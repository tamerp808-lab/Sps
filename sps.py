#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
╔══════════════════════════════════════════════════════════════════════════════╗
║  SPS v60.2 "المُسَيْطِر" — النسخة المُحسَّنة والمُصحَّحة                   ║
║  ══════════════════════════════════════════════════════════════════════════  ║
║  [FIX]  إصلاح خطأ _play_audio (وسيطة فارغة)                               ║
║  [FIX]  توحيد BACKUP_DIR في self_update / rollback                         ║
║  [FIX]  إصلاح tool_mgr غير المُهيَّأ في AgentOS                            ║
║  [FIX]  إصلاح _ollama_alive=None في status()                               ║
║  [FIX]  preexec_fn آمن على Windows                                         ║
║  [NEW]  دعم Google Colab الكامل (IS_COLAB + nest_asyncio + IPython audio)  ║
║  [NEW]  وضع الذاكرة المنخفضة IS_LOW_MEMORY (أجهزة ضعيفة)                 ║
║  [NEW]  مسارات Colab-aware (/content/sps_v60)                              ║
║  [NEW]  تعديل CFG تلقائي حسب قدرة الجهاز                                  ║
║  [NEW]  ColabSetup + nest_asyncio لحل تعارض Event Loop                     ║
║  [NEW]  --setup / --colab / --low-memory / --no-agents أعلام CLI           ║
║  [FEAT] جميع الوكلاء (Browser, OCR, Voice, Calendar, Gmail, Mobile...)    ║
║  [FEAT] نظام التحديث الذاتي الآمن (/update & /rollback)                   ║
║  [FEAT] فحص الموارد الذكي (يعتمد على Free RAM فقط)                         ║
║  [FEAT] الإصلاح الذاتي (SelfRepairLoop)                                    ║
║  [FEAT] التخطيط طويل المدى (LongTermPlanner)                               ║
║  [FEAT] التثبيت التلقائي للمكتبات (AutoInstaller)                          ║
║  [FEAT] الذاكرة المتقدمة (FAISS + NumPy fallback)                          ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""

import os, sys, json, re, ast, time, shutil, subprocess, tempfile, threading
import argparse, logging, hashlib, uuid, traceback, pickle, math
import sqlite3, platform, textwrap, importlib.util, imaplib, smtplib, email
import email.mime.text, email.mime.multipart, email.header
import urllib.request, urllib.parse, urllib.error
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional, List, Dict, Any, Tuple, Union
from collections import defaultdict, deque
import asyncio, base64, random, io, struct

# ══════════════════════════════════════════════════════════════════
#  PLATFORM DETECTION
# ══════════════════════════════════════════════════════════════════
IS_TERMUX   = "PREFIX" in os.environ and "termux" in os.environ.get("PREFIX","").lower()
IS_ANDROID  = IS_TERMUX or "ANDROID_ROOT" in os.environ
IS_WINDOWS  = platform.system() == "Windows"
IS_MAC      = platform.system() == "Darwin"
IS_LINUX    = platform.system() == "Linux" and not IS_ANDROID
IS_DESKTOP  = not IS_ANDROID

# [NEW] Google Colab detection
try:
    import google.colab as _gc          # noqa: F401
    IS_COLAB = True
except ImportError:
    IS_COLAB = (
        os.path.exists("/content") and (
            "COLAB_GPU"          in os.environ or
            "COLAB_RELEASE_TAG"  in os.environ or
            os.path.exists("/content/sample_data")
        )
    )

# [NEW] Low-memory detection: total RAM < 2 GB → weak device
def _detect_low_memory() -> bool:
    try:
        with open("/proc/meminfo") as _f:
            for _line in _f:
                if "MemTotal" in _line:
                    return int(_line.split()[1]) / 1024 < 2048
    except Exception:
        pass
    try:
        import psutil as _p
        return _p.virtual_memory().total < 2 * 1024 ** 3
    except Exception:
        pass
    return False

IS_LOW_MEMORY = _detect_low_memory()

PLATFORM_TAG = (
    "colab"    if IS_COLAB   else
    "termux"   if IS_TERMUX  else
    "android"  if IS_ANDROID else
    "windows"  if IS_WINDOWS else
    "mac"      if IS_MAC     else
    "linux"
)

# ══════════════════════════════════════════════════════════════════
#  OPTIONAL IMPORTS (graceful degradation)
# ══════════════════════════════════════════════════════════════════
try:    import aiohttp;          HAS_AIOHTTP   = True
except: HAS_AIOHTTP   = False

try:    import numpy as np;      HAS_NUMPY     = True
except: HAS_NUMPY     = False

try:    import psutil;           HAS_PSUTIL    = True
except: HAS_PSUTIL    = False

try:    import faiss;            HAS_FAISS     = True
except: HAS_FAISS     = False

try:    import resource;         HAS_RESOURCE  = True
except: HAS_RESOURCE  = False

try:
    from telegram import Update
    from telegram.ext import Application, CommandHandler, MessageHandler, filters, ContextTypes
    HAS_TELEGRAM = True
except: HAS_TELEGRAM = False

try:
    from playwright.async_api import async_playwright
    HAS_PLAYWRIGHT = True
except: HAS_PLAYWRIGHT = False

try:
    from bs4 import BeautifulSoup
    HAS_BS4 = True
except: HAS_BS4 = False

try:    import pytesseract;      HAS_TESSERACT = True
except: HAS_TESSERACT = False

try:
    from PIL import Image
    HAS_PIL = True
except: HAS_PIL = False

try:
    import faster_whisper
    HAS_WHISPER = True
except: HAS_WHISPER = False

try:
    import edge_tts
    HAS_EDGE_TTS = True
except: HAS_EDGE_TTS = False

try:
    import pyttsx3
    HAS_PYTTSX3 = True
except: HAS_PYTTSX3 = False

try:
    import speech_recognition as sr
    HAS_SPEECH_RECOGNITION = True
except: HAS_SPEECH_RECOGNITION = False

# [NEW] IPython/Colab display support
try:
    from IPython.display import Audio as _IPyAudio, display as _ipydisplay
    HAS_IPYTHON_AUDIO = True
except ImportError:
    HAS_IPYTHON_AUDIO = False

# ══════════════════════════════════════════════════════════════════
#  DIRECTORY SETUP  — [FIX] Colab-aware paths
# ══════════════════════════════════════════════════════════════════
if IS_COLAB:
    BASE_DIR = Path("/content/sps")
    DATA_DIR = Path("/content/sps_v60")
else:
    BASE_DIR = Path.home() / "sps"
    DATA_DIR = Path.home() / "sps_v60"

PROJECTS_DIR    = DATA_DIR / "projects"
MEMORY_DIR      = DATA_DIR / "memory"
LOGS_DIR        = DATA_DIR / "logs"
AGENTS_DIR      = DATA_DIR / "agents"
GOALS_DIR       = DATA_DIR / "goals"
STATE_DB        = DATA_DIR / "state.db"
BACKUP_DIR      = DATA_DIR / "backups"      # [FIX] single canonical BACKUP_DIR
TOOLS_DIR       = DATA_DIR / "tools"
SCREENSHOTS_DIR = DATA_DIR / "screenshots"
MODELS_DIR      = DATA_DIR / "models"
CALENDAR_DIR    = DATA_DIR / "calendar"
AUDIO_DIR       = DATA_DIR / "audio"
BROWSER_CACHE   = DATA_DIR / "browser_cache"

for d in [BASE_DIR, DATA_DIR, PROJECTS_DIR, MEMORY_DIR, LOGS_DIR, AGENTS_DIR,
          GOALS_DIR, BACKUP_DIR, TOOLS_DIR, SCREENSHOTS_DIR, MODELS_DIR,
          CALENDAR_DIR, AUDIO_DIR, BROWSER_CACHE]:
    d.mkdir(parents=True, exist_ok=True)

# ══════════════════════════════════════════════════════════════════
#  CONFIGURATION
# ══════════════════════════════════════════════════════════════════
CFG = {
    # ── LLM backends ──────────────────────────────────────────────
    "ollama_url":           os.getenv("OLLAMA_URL", "http://127.0.0.1:11434"),
    "ollama_model":         os.getenv("OLLAMA_MODEL", "phi4-mini"),
    "ollama_coder_model":   "phi4-mini",
    "ollama_fast_model":    "tinyllama:latest",
    "vision_model":         "moondream:latest",
    "embed_model":          "nomic-embed-text",

    "groq_api_key":         os.getenv("GROQ_API_KEY", ""),
    "groq_model":           "llama-3.3-70b-versatile",
    "groq_coder_model":     "qwen/qwen3-32b",
    "groq_fast_model":      "llama-3.1-8b-instant",
    "groq_vision_model":    "meta-llama/llama-4-scout-17b-16e-instruct",
    "groq_max_retries":     3,
    "groq_retry_base_delay":2.0,

    "openai_api_key":       os.getenv("OPENAI_API_KEY", ""),
    "openai_model":         "gpt-4o-mini",
    "openai_base_url":      os.getenv("OPENAI_BASE_URL", "https://api.openai.com/v1"),

    "llama_cli_path":       str(Path.home() / "llama.cpp/build/bin/llama-cli"),
    "llama_cpp_model":      str(MODELS_DIR / "qwen2.5-3b-instruct-q4_k_m.gguf"),

    # ── Telegram ──────────────────────────────────────────────────
    "telegram_token":       os.getenv("TELEGRAM_TOKEN", ""),

    # ── Gmail ─────────────────────────────────────────────────────
    "gmail_email":          os.getenv("GMAIL_EMAIL", ""),
    "gmail_password":       os.getenv("GMAIL_APP_PASSWORD", ""),

    # ── Voice ─────────────────────────────────────────────────────
    "whisper_model_size":   "base",
    "tts_voice":            "ar-EG-SalmaNeural",
    "tts_voice_en":         "en-US-JennyNeural",
    "voice_language":       "ar",

    # ── Browser ───────────────────────────────────────────────────
    "browser_headless":     True,
    "browser_timeout":      30000,

    # ── Resources ─────────────────────────────────────────────────
    "max_cpu_pct":              75,
    "min_battery_pct":          15,
    "sandbox_timeout":          30,
    "sandbox_max_mb":           256,
    "min_free_ram_for_tasks":   300,

    # ── Intervals ─────────────────────────────────────────────────
    "meta_cog_interval":            60,
    "goal_generation_interval":     600,
    "schedule_check_interval":      60,
    "self_repair_interval":         120,

    # ── Agent limits ──────────────────────────────────────────────
    "self_evolve":              True,
    "backup_on_modify":         True,
    "min_reward_threshold":     -0.5,
    "max_agents":               15,
    "rl_epsilon":               0.1,
    "self_repair_max_attempts": 3,

    # ── Long-term planning ────────────────────────────────────────
    "max_plan_steps":               25,
    "plan_checkpoint_interval":     5,

    # ── Sandbox security ──────────────────────────────────────────
    "forbidden_patterns": [
        r"requests\.post", r"requests\.get", r"requests\.delete", r"requests\.put",
        r"urllib\.request", r"http\.client", r"socket\.socket",
        r"os\.system", r"os\.popen", r"subprocess\.run", r"subprocess\.Popen",
        r"eval\s*\(", r"exec\s*\(", r"__import__\s*\(",
        r"open\s*\([^)]*['\"]w",
        r"shutil\.rmtree", r"shutil\.move", r"os\.remove", r"os\.rmdir",
    ],
    "allowed_apps": [
        "com.android.chrome", "com.termux",
        "org.telegram.messenger", "com.whatsapp",
        "com.google.android.youtube",
    ],
    "allowed_commands": ["am start", "input text", "input keyevent", "screencap"],

    # ── Self-update URL ───────────────────────────────────────────
    "update_url": "https://raw.githubusercontent.com/tamerp808-lab/Sps/main/sps.py",
}

# ──────────────────────────────────────────────────────────────────
#  Logging — defined early so adaptive section can log
# ──────────────────────────────────────────────────────────────────
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
    handlers=[
        logging.StreamHandler(),
        logging.FileHandler(str(LOGS_DIR / "sps_v60.log"), encoding="utf-8"),
    ]
)
log = logging.getLogger("SPS60")

# ──────────────────────────────────────────────────────────────────
#  [NEW] Adaptive CFG — weak devices & Colab
# ──────────────────────────────────────────────────────────────────
if IS_LOW_MEMORY:
    CFG.update({
        "ollama_model":               CFG["ollama_fast_model"],
        "ollama_coder_model":         CFG["ollama_fast_model"],
        "max_agents":                 5,
        "sandbox_max_mb":             128,
        "sandbox_timeout":            20,
        "min_free_ram_for_tasks":     100,
        "meta_cog_interval":          180,
        "goal_generation_interval":   1800,
        "schedule_check_interval":    120,
        "max_plan_steps":             10,
        "self_repair_max_attempts":   2,
    })
    log.info("📱 Low-memory mode activated — reduced limits applied")

if IS_COLAB:
    CFG.update({
        "browser_headless":           True,
        "min_free_ram_for_tasks":     200,
        "sandbox_timeout":            25,
    })
    log.info("☁️  Google Colab mode activated")

# ══════════════════════════════════════════════════════════════════
#  AUTO DEPENDENCY INSTALLER  — [NEW] Colab support
# ══════════════════════════════════════════════════════════════════
class AutoInstaller:
    """Install Python packages and system tools based on current platform."""

    PY_PACKAGES = {
        "aiohttp":              {"pip": "aiohttp"},
        "numpy":                {"pip": "numpy"},
        "faiss":                {"pip": "faiss-cpu"},
        "bs4":                  {"pip": "beautifulsoup4"},
        "PIL":                  {"pip": "Pillow"},
        "pytesseract":          {"pip": "pytesseract"},
        "faster_whisper":       {"pip": "faster-whisper"},
        "edge_tts":             {"pip": "edge-tts"},
        "pyttsx3":              {"pip": "pyttsx3"},
        "speech_recognition":   {"pip": "SpeechRecognition"},
        "telegram":             {"pip": "python-telegram-bot"},
        "playwright":           {"pip": "playwright", "post": "playwright install chromium"},
        "icalendar":            {"pip": "icalendar"},
        "nest_asyncio":         {"pip": "nest_asyncio"},
    }

    SYSTEM_PKGS = {
        "termux":  {
            "ffmpeg":     "pkg install ffmpeg -y",
            "tesseract":  "pkg install tesseract -y",
            "espeak-ng":  "pkg install espeak-ng -y",
        },
        "linux":   {
            "ffmpeg":     "sudo apt-get install -y ffmpeg",
            "tesseract":  "sudo apt-get install -y tesseract-ocr",
            "espeak-ng":  "sudo apt-get install -y espeak-ng",
        },
        "mac":     {
            "ffmpeg":     "brew install ffmpeg",
            "tesseract":  "brew install tesseract",
            "espeak-ng":  "brew install espeak",
        },
        "windows": {
            "ffmpeg":     "winget install ffmpeg",
            "tesseract":  "winget install UB-Mannheim.TesseractOCR",
        },
        # [NEW] Colab uses apt-get without sudo
        "colab":   {
            "ffmpeg":     "apt-get install -y ffmpeg",
            "tesseract":  "apt-get install -y tesseract-ocr",
            "espeak-ng":  "apt-get install -y espeak-ng",
        },
    }

    # [NEW] Minimal package set for Colab / weak devices
    COLAB_ESSENTIAL = [
        "aiohttp", "beautifulsoup4", "edge-tts",
        "nest_asyncio", "python-telegram-bot",
    ]

    @classmethod
    def install_python_package(cls, name: str) -> bool:
        info = cls.PY_PACKAGES.get(name)
        if not info:
            return False
        pkg = info["pip"]
        log.info(f"📦 pip install {pkg}")
        try:
            result = subprocess.run(
                [sys.executable, "-m", "pip", "install", pkg, "--quiet",
                 "--break-system-packages"],
                capture_output=True, text=True, timeout=120
            )
            if result.returncode == 0:
                log.info(f"✅ {pkg} installed")
                post = info.get("post")
                if post:
                    subprocess.run(post.split(), capture_output=True, timeout=120)
                return True
            log.warning(f"pip install {pkg} failed: {result.stderr[:200]}")
        except Exception as e:
            log.warning(f"pip install error: {e}")
        return False

    @classmethod
    def install_system_tool(cls, tool: str) -> bool:
        pkgs = cls.SYSTEM_PKGS.get(PLATFORM_TAG, {})
        cmd = pkgs.get(tool)
        if not cmd:
            return False
        log.info(f"📦 system install {tool}: {cmd}")
        try:
            result = subprocess.run(cmd.split(), capture_output=True, text=True, timeout=180)
            return result.returncode == 0
        except Exception:
            return False

    @classmethod
    def ensure(cls, module_name: str) -> bool:
        """Ensure a Python module is available, install if not."""
        if importlib.util.find_spec(module_name):
            return True
        return cls.install_python_package(module_name)

    @classmethod
    def setup_colab(cls):
        """[NEW] Install essential packages for Google Colab."""
        log.info("☁️  Setting up SPS for Google Colab...")
        for pkg in cls.COLAB_ESSENTIAL:
            subprocess.run(
                [sys.executable, "-m", "pip", "install", pkg, "-q"],
                capture_output=True, timeout=120
            )
        # Apply nest_asyncio to fix Colab event loop conflict
        try:
            import nest_asyncio
            nest_asyncio.apply()
            log.info("✅ nest_asyncio applied")
        except ImportError:
            pass
        log.info("✅ Colab setup complete")

    @classmethod
    def setup_all(cls):
        """[NEW] Install all optional packages (for full capability)."""
        log.info("📦 Installing all optional packages...")
        for name in cls.PY_PACKAGES:
            if not importlib.util.find_spec(name):
                cls.install_python_package(name)
        log.info("✅ Full setup complete")


# ══════════════════════════════════════════════════════════════════
#  RESOURCE MONITOR (Free RAM based)
# ══════════════════════════════════════════════════════════════════
class ResourceMonitor:
    @staticmethod
    def cpu_percent() -> float:
        if HAS_PSUTIL:
            try: return psutil.cpu_percent(interval=0.3)
            except: pass
        try:
            with open("/proc/stat") as f: f1 = f.readline().split()
            time.sleep(0.3)
            with open("/proc/stat") as f: f2 = f.readline().split()
            idle1, total1 = int(f1[4]), sum(int(x) for x in f1[1:])
            idle2, total2 = int(f2[4]), sum(int(x) for x in f2[1:])
            dt = total2 - total1
            return 100.0 * (1 - (idle2 - idle1) / dt) if dt else 0.0
        except: return 0.0

    @staticmethod
    def ram_used_mb() -> float:
        if HAS_PSUTIL:
            try: return psutil.virtual_memory().used / (1024*1024)
            except: pass
        try:
            with open("/proc/meminfo") as f: lines = f.readlines()
            total = int([l for l in lines if "MemTotal"     in l][0].split()[1])
            avail = int([l for l in lines if "MemAvailable" in l][0].split()[1]) \
                    if any("MemAvailable" in l for l in lines) else 0
            return (total - avail) / 1024
        except: return 0.0

    @staticmethod
    def available_ram_mb() -> float:
        if HAS_PSUTIL:
            try: return psutil.virtual_memory().available / (1024*1024)
            except: pass
        try:
            with open("/proc/meminfo") as f: lines = f.readlines()
            if any("MemAvailable" in l for l in lines):
                return int([l for l in lines if "MemAvailable" in l][0].split()[1]) / 1024
            free    = int([l for l in lines if "MemFree"  in l][0].split()[1])
            buffers = int([l for l in lines if "Buffers"  in l][0].split()[1]) if any("Buffers" in l for l in lines) else 0
            cached  = int([l for l in lines if "Cached"   in l][0].split()[1]) if any("Cached"  in l for l in lines) else 0
            return (free + buffers + cached) / 1024
        except: return 0.0

    @staticmethod
    def battery_pct() -> int:
        if HAS_PSUTIL:
            try:
                bat = psutil.sensors_battery()
                if bat: return int(bat.percent)
            except: pass
        try:
            res = subprocess.run(["termux-battery-status"], capture_output=True, text=True, timeout=2)
            if res.returncode == 0:
                return int(json.loads(res.stdout).get("percentage", 100))
        except: pass
        for p in Path("/sys/class/power_supply").glob("*/capacity"):
            try: return int(p.read_text().strip())
            except: pass
        return 100   # assume full if unknown (e.g. Colab, desktop)

    @classmethod
    def safe(cls, min_free_ram: float = None) -> bool:
        if min_free_ram is None:
            min_free_ram = CFG.get("min_free_ram_for_tasks", 300)
        try:
            cpu      = cls.cpu_percent()
            bat      = cls.battery_pct()
            free_ram = cls.available_ram_mb()
            ok = (
                cpu < CFG["max_cpu_pct"] and
                bat > CFG["min_battery_pct"] and
                free_ram >= min_free_ram
            )
            if not ok:
                reasons = []
                if cpu >= CFG["max_cpu_pct"]:       reasons.append(f"CPU {cpu:.1f}%")
                if bat <= CFG["min_battery_pct"]:   reasons.append(f"Battery {bat}%")
                if free_ram < min_free_ram:          reasons.append(f"Free RAM {free_ram:.0f}MB < {min_free_ram}MB")
                log.warning(f"Resources unsafe: {', '.join(reasons)}")
            return ok
        except: return True

    @classmethod
    def stats(cls) -> Dict:
        return {
            "cpu":        cls.cpu_percent(),
            "ram_used_mb":cls.ram_used_mb(),
            "ram_free_mb":cls.available_ram_mb(),
            "battery":    cls.battery_pct(),
            "low_memory": IS_LOW_MEMORY,
            "platform":   PLATFORM_TAG,
        }


# ══════════════════════════════════════════════════════════════════
#  STEALTH MODE
# ══════════════════════════════════════════════════════════════════
class StealthMode:
    """Minimal footprint: rotate logs, wipe old files, no telemetry."""

    @staticmethod
    def wipe_old_logs():
        ttl = timedelta(hours=24)
        for log_file in LOGS_DIR.glob("*.log"):
            try:
                age = datetime.now() - datetime.fromtimestamp(log_file.stat().st_mtime)
                if age > ttl:
                    log_file.unlink()
            except Exception: pass

    @staticmethod
    def wipe_browser_cache():
        try:
            shutil.rmtree(str(BROWSER_CACHE), ignore_errors=True)
            BROWSER_CACHE.mkdir(exist_ok=True)
        except Exception: pass

    @staticmethod
    def wipe_temp_audio():
        for f in AUDIO_DIR.glob("*.mp3"):
            try: f.unlink()
            except: pass

    @classmethod
    def run_cycle(cls):
        cls.wipe_old_logs()
        cls.wipe_browser_cache()
        cls.wipe_temp_audio()


# ══════════════════════════════════════════════════════════════════
#  HTTP HELPERS
# ══════════════════════════════════════════════════════════════════
def _urllib_post_json(url, payload, headers=None, timeout=60):
    data = json.dumps(payload).encode()
    h = {"Content-Type": "application/json"}
    if headers: h.update(headers)
    req = urllib.request.Request(url, data=data, headers=h, method="POST")
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.loads(resp.read().decode())

async def _async_post_json(url, payload, headers=None, timeout=60):
    if HAS_AIOHTTP:
        async with aiohttp.ClientSession() as s:
            async with s.post(url, json=payload, headers=headers or {},
                              timeout=aiohttp.ClientTimeout(total=timeout)) as r:
                if r.status not in (200, 201):
                    text = await r.text()
                    raise RuntimeError(f"HTTP {r.status}: {text[:300]}")
                return await r.json()
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(
        None, lambda: _urllib_post_json(url, payload, headers, timeout)
    )

async def _async_get_json(url, timeout=10):
    if HAS_AIOHTTP:
        async with aiohttp.ClientSession() as s:
            async with s.get(url, timeout=aiohttp.ClientTimeout(total=timeout)) as r:
                if r.status != 200: raise RuntimeError(f"HTTP {r.status}")
                return await r.json()
    loop = asyncio.get_event_loop()
    def _get():
        with urllib.request.urlopen(url, timeout=timeout) as r:
            return json.loads(r.read().decode())
    return await loop.run_in_executor(None, _get)

async def _async_get_text(url, timeout=20):
    if HAS_AIOHTTP:
        async with aiohttp.ClientSession() as s:
            async with s.get(url, timeout=aiohttp.ClientTimeout(total=timeout),
                             headers={"User-Agent": "Mozilla/5.0 SPS/60"}) as r:
                return await r.text()
    loop = asyncio.get_event_loop()
    def _get():
        req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 SPS/60"})
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.read().decode(errors="replace")
    return await loop.run_in_executor(None, _get)


# ══════════════════════════════════════════════════════════════════
#  VECTOR MEMORY (numpy fallback + FAISS)
# ══════════════════════════════════════════════════════════════════
class NumpyVectorMemory:
    def __init__(self, dim=384):
        self.dim = dim; self.vectors = []; self.records = []
        self._load()

    def _load(self):
        m, v = MEMORY_DIR/"np_meta.pkl", MEMORY_DIR/"np_vecs.pkl"
        if m.exists() and v.exists():
            try:
                with open(m,"rb") as f: self.records = pickle.load(f)
                with open(v,"rb") as f: self.vectors  = pickle.load(f)
            except: self.records=[]; self.vectors=[]

    def _save(self):
        try:
            with open(MEMORY_DIR/"np_meta.pkl","wb") as f: pickle.dump(self.records,f)
            with open(MEMORY_DIR/"np_vecs.pkl", "wb") as f: pickle.dump(self.vectors,f)
        except: pass

    def _cos(self, a, b):
        if HAS_NUMPY:
            a, b = np.array(a,dtype=np.float32), np.array(b,dtype=np.float32)
            return float(np.dot(a,b)/(np.linalg.norm(a)*np.linalg.norm(b)+1e-8))
        dot = sum(x*y for x,y in zip(a,b))
        return dot/(math.sqrt(sum(x*x for x in a))*math.sqrt(sum(x*x for x in b))+1e-8)

    def add(self, vec, record):
        self.vectors.append(vec); self.records.append(record); self._save()

    def search(self, vec, top_k=3, min_reward=0.5):
        if not self.vectors: return []
        sims = sorted([(self._cos(vec,v),i) for i,v in enumerate(self.vectors)], key=lambda x:-x[0])
        return [{**dict(self.records[i]), "similarity":s} for s,i in sims[:top_k]
                if self.records[i].get("reward",0) >= min_reward]


class FAISSVectorMemory:
    def __init__(self, llm, dim=384):
        self.llm=llm; self.dim=dim; self.records=[]; self._fb=NumpyVectorMemory(dim)
        if HAS_FAISS:
            try: self.index=faiss.IndexFlatIP(dim); self._load_faiss()
            except: self.index=None
        else: self.index=None

    def _load_faiss(self):
        p, m = MEMORY_DIR/"faiss.index", MEMORY_DIR/"faiss_meta.pkl"
        if p.exists() and m.exists():
            try:
                self.index=faiss.read_index(str(p))
                with open(m,"rb") as f: self.records=pickle.load(f)
            except: pass

    def _save_faiss(self):
        if self.index:
            try:
                faiss.write_index(self.index, str(MEMORY_DIR/"faiss.index"))
                with open(MEMORY_DIR/"faiss_meta.pkl","wb") as f: pickle.dump(self.records,f)
            except: pass

    async def add(self, task, code, reward):
        emb = await self.llm.embed(task)
        if emb is None: return
        rec = {"task":task,"code":code,"reward":reward}
        if HAS_FAISS and self.index and HAS_NUMPY:
            a=np.array(emb,dtype=np.float32); a/=np.linalg.norm(a)+1e-8
            if a.shape[0]==self.dim:
                self.index.add(a.reshape(1,-1)); self.records.append(rec); self._save_faiss(); return
        self._fb.add(emb, rec)

    async def search(self, task, top_k=3, min_reward=0.5):
        emb = await self.llm.embed(task)
        if emb is None: return []
        if HAS_FAISS and self.index and self.index.ntotal>0 and HAS_NUMPY:
            a=np.array(emb,dtype=np.float32); a/=np.linalg.norm(a)+1e-8
            if a.shape[0]==self.dim:
                scores,idxs=self.index.search(a.reshape(1,-1),min(top_k,self.index.ntotal))
                return [{**dict(self.records[i]),"similarity":float(s)}
                        for s,i in zip(scores[0],idxs[0]) if i!=-1
                        and self.records[i].get("reward",0)>=min_reward]
        return self._fb.search(emb, top_k, min_reward)


# ══════════════════════════════════════════════════════════════════
#  DATABASE (extended schema)
# ══════════════════════════════════════════════════════════════════
class DB:
    def __init__(self):
        self.conn = sqlite3.connect(str(STATE_DB), check_same_thread=False)
        self.conn.row_factory = sqlite3.Row
        self._lock = threading.Lock()
        self._init_schema()

    def _init_schema(self):
        stmts = [
            "CREATE TABLE IF NOT EXISTS tasks (id TEXT PRIMARY KEY, desc TEXT, status TEXT, created TIMESTAMP DEFAULT CURRENT_TIMESTAMP, completed TIMESTAMP, result TEXT, reward REAL DEFAULT 0, error_context TEXT)",
            "CREATE TABLE IF NOT EXISTS memory (key TEXT PRIMARY KEY, code TEXT, task TEXT, reward REAL DEFAULT 0, usage INTEGER DEFAULT 0, created TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS agents (role TEXT PRIMARY KEY, code TEXT, created TIMESTAMP DEFAULT CURRENT_TIMESTAMP, success_count INTEGER DEFAULT 0, fail_count INTEGER DEFAULT 0, avg_reward REAL DEFAULT 0.0, embedding BLOB)",
            "CREATE TABLE IF NOT EXISTS failure_patterns (id INTEGER PRIMARY KEY AUTOINCREMENT, error_signature TEXT UNIQUE, root_cause TEXT, fix_strategy TEXT, occurrence INTEGER DEFAULT 1, last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS meta_metrics (id INTEGER PRIMARY KEY AUTOINCREMENT, timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, success_rate REAL, avg_reward REAL, agent_count INTEGER)",
            "CREATE TABLE IF NOT EXISTS long_term_memory (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, embedding BLOB, importance REAL DEFAULT 0, created TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS goals (id TEXT PRIMARY KEY, description TEXT, status TEXT, priority INTEGER DEFAULT 5, created TIMESTAMP DEFAULT CURRENT_TIMESTAMP, parent_id TEXT)",
            "CREATE TABLE IF NOT EXISTS schedule (id TEXT PRIMARY KEY, interval_seconds INTEGER DEFAULT 3600, task_desc TEXT, last_run TIMESTAMP, enabled INTEGER DEFAULT 1)",
            "CREATE TABLE IF NOT EXISTS episodes (id INTEGER PRIMARY KEY AUTOINCREMENT, task TEXT, steps TEXT, outcome TEXT, timestamp TEXT)",
            "CREATE TABLE IF NOT EXISTS plans (id TEXT PRIMARY KEY, goal TEXT, steps TEXT, current_step INTEGER DEFAULT 0, status TEXT DEFAULT 'active', created TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS repairs (id INTEGER PRIMARY KEY AUTOINCREMENT, task TEXT, error TEXT, attempts INTEGER, fixed INTEGER DEFAULT 0, timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE IF NOT EXISTS calendar (id TEXT PRIMARY KEY, title TEXT, start_dt TEXT, end_dt TEXT, location TEXT, description TEXT, ics_file TEXT)",
            "CREATE TABLE IF NOT EXISTS browser_history (id INTEGER PRIMARY KEY AUTOINCREMENT, url TEXT, title TEXT, visited TIMESTAMP DEFAULT CURRENT_TIMESTAMP, purge_after TIMESTAMP)",
        ]
        with self._lock:
            for s in stmts: self.conn.execute(s)
            self.conn.commit()

    def _exec(self, sql, args=()):
        with self._lock:
            self.conn.execute(sql, args)
            self.conn.commit()

    def _fetch(self, sql, args=()):
        return [dict(r) for r in self.conn.execute(sql, args).fetchall()]

    def save_task(self, tid, desc, status, result=None, reward=0.0):
        with self._lock:
            self.conn.execute(
                "INSERT OR REPLACE INTO tasks (id,desc,status,completed,result,reward) VALUES (?,?,?,?,?,?)",
                (tid, desc, status, datetime.now().isoformat() if status=="done" else None,
                 json.dumps(result) if result else None, reward))
            self.conn.commit()

    def add_memory(self, key, code, task, reward):
        self._exec(
            "INSERT INTO memory (key,code,task,reward,usage) VALUES (?,?,?,?,1) "
            "ON CONFLICT(key) DO UPDATE SET usage=usage+1, reward=MAX(reward,excluded.reward)",
            (key, code, task, reward)
        )

    def save_agent(self, role, code, embedding=None):
        emb = pickle.dumps(embedding) if embedding else None
        self._exec("INSERT OR REPLACE INTO agents (role,code,embedding) VALUES (?,?,?)", (role,code,emb))

    def list_agents(self): return self._fetch("SELECT * FROM agents")
    def get_agent_code(self, role):
        rows = self._fetch("SELECT code FROM agents WHERE role=?", (role,))
        return rows[0]["code"] if rows else None

    def update_agent_stats(self, role, success, reward):
        self._exec(
            "UPDATE agents SET success_count=success_count+?, fail_count=fail_count+?, "
            "avg_reward=(avg_reward*(success_count+fail_count)+?)/(success_count+fail_count+1) WHERE role=?",
            (1 if success else 0, 0 if success else 1, reward, role)
        )

    def add_goal(self, desc, priority=5, parent_id=None):
        gid = uuid.uuid4().hex
        self._exec("INSERT INTO goals (id,description,status,priority,parent_id) VALUES (?,?,?,?,?)",
                   (gid,desc,"pending",priority,parent_id))
        return gid

    def get_goals(self, status=None):
        q = "SELECT * FROM goals" + (" WHERE status=?" if status else "") + " ORDER BY priority DESC, created ASC"
        return self._fetch(q, (status,) if status else ())

    def add_metric(self, sr, ar, ac):
        self._exec("INSERT INTO meta_metrics (success_rate,avg_reward,agent_count) VALUES (?,?,?)", (sr,ar,ac))

    def recent_metrics(self, n=20):
        return self._fetch("SELECT * FROM meta_metrics ORDER BY timestamp DESC LIMIT ?", (n,))

    def top_memories(self, n=20):
        return self._fetch("SELECT * FROM memory WHERE usage>1 AND reward>0.3 ORDER BY reward DESC LIMIT ?", (n,))

    def add_failure(self, sig, root, fix):
        self._exec(
            "INSERT INTO failure_patterns (error_signature,root_cause,fix_strategy) VALUES (?,?,?) "
            "ON CONFLICT(error_signature) DO UPDATE SET occurrence=occurrence+1,last_seen=CURRENT_TIMESTAMP",
            (sig[:200],root,fix)
        )

    def get_due_schedules(self):
        rows = self._fetch("SELECT * FROM schedule WHERE enabled=1")
        now = datetime.now()
        return [r for r in rows
                if r["last_run"] is None or
                (datetime.now() - datetime.fromisoformat(str(r["last_run"]))).total_seconds()
                >= r.get("interval_seconds", 3600)]

    def update_schedule_last_run(self, sid):
        self._exec("UPDATE schedule SET last_run=? WHERE id=?", (datetime.now(), sid))

    def save_plan(self, plan_id, goal, steps, current=0, status="active"):
        self._exec(
            "INSERT OR REPLACE INTO plans (id,goal,steps,current_step,status,updated) VALUES (?,?,?,?,?,?)",
            (plan_id, goal, json.dumps(steps), current, status, datetime.now().isoformat())
        )

    def get_plan(self, plan_id):
        rows = self._fetch("SELECT * FROM plans WHERE id=?", (plan_id,))
        if rows:
            r = rows[0]; r["steps"] = json.loads(r["steps"]); return r
        return None

    def get_active_plans(self):
        plans = self._fetch("SELECT * FROM plans WHERE status='active' ORDER BY created ASC")
        for p in plans: p["steps"] = json.loads(p["steps"])
        return plans

    def update_plan_step(self, plan_id, step, status="active"):
        self._exec("UPDATE plans SET current_step=?,status=?,updated=? WHERE id=?",
                   (step, status, datetime.now().isoformat(), plan_id))

    def add_calendar_event(self, title, start_dt, end_dt=None, location="", description="", ics_file=""):
        eid = uuid.uuid4().hex
        self._exec(
            "INSERT INTO calendar (id,title,start_dt,end_dt,location,description,ics_file) VALUES (?,?,?,?,?,?,?)",
            (eid,title,start_dt,end_dt or "",location,description,ics_file)
        )
        return eid

    def get_upcoming_events(self, days=7):
        cutoff = (datetime.now() + timedelta(days=days)).isoformat()
        return self._fetch("SELECT * FROM calendar WHERE start_dt<=? ORDER BY start_dt ASC", (cutoff,))

    def add_repair(self, task, error, attempts, fixed):
        self._exec("INSERT INTO repairs (task,error,attempts,fixed) VALUES (?,?,?,?)",
                   (task[:300], error[:500], attempts, 1 if fixed else 0))

    def add_browser_visit(self, url, title):
        purge = (datetime.now() + timedelta(hours=24)).isoformat()
        self._exec("INSERT INTO browser_history (url,title,purge_after) VALUES (?,?,?)",
                   (url[:500],title[:200],purge))

    def purge_browser_history(self):
        self._exec("DELETE FROM browser_history WHERE purge_after IS NOT NULL AND purge_after < ?",
                   (datetime.now().isoformat(),))


# ══════════════════════════════════════════════════════════════════
#  LLM — same fallback chain with Groq retry
# ══════════════════════════════════════════════════════════════════
class LLM:
    SYSTEM_PROMPTS = {
        "coder":    "You are an expert Python coder. Output ONLY valid Python code, no markdown fences, no explanations.",
        "reviewer": 'You are a code reviewer. Output ONLY JSON: {"approve":bool,"issues":[str]}',
        "debugger": "You are a debugger. Output ONLY the corrected Python code. No markdown.",
        "planner":  "You are a strategic planner. Output ONLY valid JSON.",
        "analyst":  "You are a root-cause analyst. Output ONLY JSON.",
        "browser":  "You are a web automation expert. Output JSON commands for browser actions.",
        "calendar": "You are a calendar assistant. Parse dates and create structured JSON events.",
        "repair":   "You are a code repair specialist. Fix errors and return working code only.",
        "default":  "You are a helpful AI assistant.",
    }

    def __init__(self, db):
        self.db = db
        self._ollama_alive: Optional[bool] = None
        self._ollama_ts: float = 0.0

    async def _probe_ollama(self) -> bool:
        if self._ollama_alive is not None and time.time()-self._ollama_ts < 30:
            return self._ollama_alive
        try:
            await _async_get_json(CFG["ollama_url"]+"/api/tags", timeout=4)
            self._ollama_alive = True
        except: self._ollama_alive = False
        self._ollama_ts = time.time()
        return self._ollama_alive

    def _has(self, key): return bool(CFG.get(key))

    def _has_llama_cli(self):
        return os.path.exists(CFG["llama_cli_path"]) and os.path.exists(CFG["llama_cpp_model"])

    async def chat(self, prompt, role="default", history=None, images=None):
        sys_p = self.SYSTEM_PROMPTS.get(role, self.SYSTEM_PROMPTS["default"])
        msgs  = [{"role":"system","content":sys_p}] + (history or [])
        msgs.append({"role":"user","content":prompt})

        if self._has("groq_api_key"):
            model = (
                CFG["groq_coder_model"] if role=="coder" else
                CFG["groq_fast_model"]  if role in ("reviewer","analyst","debugger","repair") else
                CFG["groq_model"]
            )
            try:
                r = await self._groq_chat(msgs, model)
                if r: return r
            except Exception as e: log.warning(f"Groq fail: {e}")

        if self._has("openai_api_key"):
            try:
                r = await self._oai_chat(msgs, CFG["openai_base_url"]+"/chat/completions",
                                          CFG["openai_model"], CFG["openai_api_key"])
                if r: return r
            except Exception as e: log.warning(f"OpenAI fail: {e}")

        if await self._probe_ollama():
            try:
                m = CFG["ollama_coder_model"] if role=="coder" else CFG["ollama_model"]
                if images:
                    r = await self._ollama_vision(prompt, sys_p, images)
                else:
                    r = await self._ollama_chat(msgs, m)
                if r: return r
            except Exception as e: log.warning(f"Ollama fail: {e}")

        if self._has_llama_cli():
            try:
                fp = f"{sys_p}\n\nUser: {prompt}\nAssistant:"
                r = await self._llama_cli(fp)
                if r: return r
            except Exception as e: log.warning(f"llama-cli fail: {e}")

        return self._stub(prompt, role)

    async def _groq_chat(self, msgs, model):
        url = "https://api.groq.com/openai/v1/chat/completions"
        h   = {"Authorization": f"Bearer {CFG['groq_api_key']}"}
        pl  = {"model":model,"messages":msgs,"max_tokens":2048,"temperature":0.3}
        for i in range(CFG["groq_max_retries"]):
            try:
                d = await _async_post_json(url, pl, h, 60)
                if "choices" not in d: raise RuntimeError(str(d.get("error",d)))
                return d["choices"][0]["message"]["content"].strip()
            except RuntimeError as e:
                if "429" in str(e) or "rate_limit" in str(e).lower():
                    await asyncio.sleep(CFG["groq_retry_base_delay"]*(2**i)); continue
                raise
        raise RuntimeError("Groq retries exhausted")

    async def _oai_chat(self, msgs, url, model, key):
        d = await _async_post_json(url,
            {"model":model,"messages":msgs,"max_tokens":2048,"temperature":0.3},
            {"Authorization":f"Bearer {key}"}, 60)
        if "choices" not in d: raise RuntimeError(str(d.get("error",d)))
        return d["choices"][0]["message"]["content"].strip()

    async def _ollama_chat(self, msgs, model):
        d = await _async_post_json(CFG["ollama_url"]+"/api/chat",
                                    {"model":model,"messages":msgs,"stream":False}, timeout=120)
        return d["message"]["content"].strip()

    async def _ollama_vision(self, prompt, sys_p, images):
        imgs_b64 = []
        for p in images:
            if os.path.exists(p):
                with open(p,"rb") as f: imgs_b64.append(base64.b64encode(f.read()).decode())
        msgs = [{"role":"user","content":[{"type":"text","text":f"{sys_p}\n{prompt}"}]+
                 [{"type":"image_url","image_url":{"url":f"data:image/jpeg;base64,{b}"}} for b in imgs_b64]}]
        return await self._ollama_chat(msgs, CFG["vision_model"])

    async def _llama_cli(self, prompt):
        cmd = [CFG["llama_cli_path"],"-m",CFG["llama_cpp_model"],"-p",prompt,
               "-n","512","--temp","0.3","--simple-io","--no-display-prompt"]
        proc = await asyncio.create_subprocess_exec(
            *cmd, stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE
        )
        out,_ = await asyncio.wait_for(proc.communicate(), timeout=60)
        return out.decode(errors="replace").strip()

    async def embed(self, text):
        if await self._probe_ollama():
            try:
                d = await _async_post_json(CFG["ollama_url"]+"/api/embeddings",
                                            {"model":CFG["embed_model"],"prompt":text},timeout=30)
                if d.get("embedding"): return d["embedding"]
            except: pass
        seed = int.from_bytes(hashlib.sha256(text.encode()).digest()[:8],"big")
        rng  = random.Random(seed)
        dim  = 384
        if HAS_NUMPY:
            v=np.array([rng.gauss(0,1) for _ in range(dim)],dtype=np.float32)
            v/=np.linalg.norm(v)+1e-8; return v.tolist()
        v=[rng.gauss(0,1) for _ in range(dim)]
        n=math.sqrt(sum(x*x for x in v))+1e-8; return [x/n for x in v]

    def _stub(self, p, role):
        if role=="coder":    return "print('SPS v60.2 DOMINATOR')"
        if role=="planner":  return json.dumps([{"step":1,"task":p[:80]}])
        if role=="reviewer": return '{"approve":true,"issues":[]}'
        return "OK"

    async def check(self):
        return (self._has("groq_api_key") or self._has("openai_api_key") or
                await self._probe_ollama() or self._has_llama_cli())


# ══════════════════════════════════════════════════════════════════
#  SECURE SANDBOX  — [FIX] preexec_fn safe on Windows
# ══════════════════════════════════════════════════════════════════
class SecureSandbox:
    @staticmethod
    def quick_lint(code):
        try: ast.parse(code)
        except SyntaxError as e: return False, f"SyntaxError: {e}"
        for p in CFG.get("forbidden_patterns",[]):
            if re.search(p,code): return False, f"Forbidden: {p}"
        try:
            for node in ast.walk(ast.parse(code)):
                if isinstance(node,ast.Import):
                    for a in node.names:
                        if a.name in ["requests","urllib","http","socket","os","subprocess","shutil"]:
                            return False, f"Forbidden import: {a.name}"
                elif isinstance(node,ast.ImportFrom):
                    if node.module in ["requests","urllib","http","socket","os","subprocess","shutil"]:
                        return False, f"Forbidden import from: {node.module}"
                elif isinstance(node,ast.Call):
                    if isinstance(node.func,ast.Name) and node.func.id in ["eval","exec","__import__"]:
                        return False, f"Forbidden call: {node.func.id}"
        except: pass
        return True, "ok"

    @staticmethod
    def _set_limits():
        if not HAS_RESOURCE: return
        for limit, val in [(resource.RLIMIT_CPU, CFG["sandbox_timeout"]),
                           (resource.RLIMIT_AS,  CFG["sandbox_max_mb"]*1024*1024)]:
            try: resource.setrlimit(limit,(val,val))
            except: pass

    async def run(self, code, timeout=None):
        ok, err = self.quick_lint(code)
        if not ok: return False, err, 0.0, 0.0
        timeout = timeout or CFG["sandbox_timeout"]
        with tempfile.NamedTemporaryFile(suffix=".py",mode="w",encoding="utf-8",delete=False) as f:
            f.write(code); fp = f.name
        try:
            cmd = [sys.executable, fp]
            if shutil.which("timeout"): cmd = ["timeout", str(timeout)] + cmd
            # [FIX] preexec_fn only on POSIX (not Windows)
            use_preexec = HAS_RESOURCE and not IS_WINDOWS
            proc = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env={**os.environ,"PYTHONDONTWRITEBYTECODE":"1"},
                preexec_fn=SecureSandbox._set_limits if use_preexec else None
            )
            try:
                out, err2 = await asyncio.wait_for(proc.communicate(), timeout=timeout+2)
            except asyncio.TimeoutError:
                proc.kill(); await proc.communicate()
                return False,"TimeoutError",0.0,0.2
            o = (out.decode(errors="replace")+err2.decode(errors="replace")).strip()
            ok = proc.returncode == 0
            return ok, o[:2000], (min(1.0,0.7+len(o)/500) if ok else 0.0), 0.1
        except Exception as e: return False,str(e),0.0,0.1
        finally:
            try: os.unlink(fp)
            except: pass


# ══════════════════════════════════════════════════════════════════
#  SWARM BUS
# ══════════════════════════════════════════════════════════════════
class SwarmBus:
    def __init__(self): self._h={}; self._r={}

    def register(self, aid):
        q=asyncio.Queue(maxsize=100); self._h[aid]=q; return q

    def unregister(self, aid): self._h.pop(aid,None)

    async def send(self, aid, msg):
        q = self._h.get(aid)
        if q:
            try: await asyncio.wait_for(q.put(msg),timeout=5.0); return True
            except asyncio.TimeoutError: return False
        return False

    async def request(self, aid, action, payload, timeout=30.0):
        rid=uuid.uuid4().hex; rq=asyncio.Queue(maxsize=1); self._r[rid]=rq
        if not await self.send(aid,{"action":action,"payload":payload,"reply_id":rid}):
            del self._r[rid]; return None
        try: return await asyncio.wait_for(rq.get(),timeout=timeout)
        except asyncio.TimeoutError: return None
        finally: self._r.pop(rid,None)

    async def reply(self, rid, data):
        q = self._r.get(rid)
        if q:
            try: q.put_nowait(data)
            except asyncio.QueueFull: pass


# ══════════════════════════════════════════════════════════════════
#  BASE AGENT
# ══════════════════════════════════════════════════════════════════
class BaseAgent:
    def __init__(self, aid, role, llm, bus, sandbox, db):
        self.id=aid; self.role=role; self.llm=llm; self.bus=bus
        self.sandbox=sandbox; self.db=db
        self._queue=None; self._task=None; self._alive=False
        self.heartbeat=datetime.now()

    async def start(self):
        self._queue=self.bus.register(self.id)
        self._alive=True
        self._task=asyncio.create_task(self._loop())

    async def stop(self):
        self._alive=False; self.bus.unregister(self.id)
        if self._task: self._task.cancel()

    async def _loop(self):
        while self._alive:
            try:
                msg=await asyncio.wait_for(self._queue.get(),timeout=1.0)
                self.heartbeat=datetime.now()
                result=await self._handle(msg.get("action"),msg.get("payload",{}))
                rid=msg.get("reply_id")
                if rid: await self.bus.reply(rid, result or {})
            except asyncio.TimeoutError: continue
            except asyncio.CancelledError: break
            except Exception as e: log.error(f"Agent {self.id}: {e}")

    async def _handle(self, action, payload): raise NotImplementedError


# ══════════════════════════════════════════════════════════════════
#  CORE AGENTS
# ══════════════════════════════════════════════════════════════════
class CoderAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="write_code":
            code=await self.llm.chat(
                f"Write Python code for: {payload.get('task','')}. Style: {payload.get('style','simple')}. Output raw Python only.",
                role="coder")
            return {"code": re.sub(r"```python|```","",code).strip()}
        return {}

class ReviewerAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="review":
            code=payload.get("code","")
            ok,msg=SecureSandbox.quick_lint(code)
            if not ok: return {"approve":False,"issues":[msg]}
            resp=await self.llm.chat(f"Review code:\n{code[:1500]}", role="reviewer")
            try:
                m=re.search(r"\{[^{}]*\}",resp,re.S)
                return json.loads(m.group()) if m else {"approve":True,"issues":[]}
            except: return {"approve":True,"issues":[]}
        return {}

class DebuggerAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="debug":
            fix=await self.llm.chat(
                f"Fix error: {payload.get('error','')}\nCode:\n{payload.get('code','')[:1500]}",
                role="debugger")
            return {"fix": re.sub(r"```python|```","",fix).strip()}
        return {}

class PlannerAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="plan":
            resp=await self.llm.chat(f"Plan for: {payload.get('goal','')}. Output JSON array of steps.", role="planner")
            try:
                m=re.search(r"\[.*?\]",resp,re.S)
                return {"steps":json.loads(m.group()) if m else [{"step":1,"task":payload.get("goal","")}]}
            except: return {"steps":[{"step":1,"task":payload.get("goal","")}]}
        return {}

class VisionAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="analyze_image":
            img=payload.get("image_path")
            if not img or not os.path.exists(img): return {"success":False,"error":"Image not found"}
            desc=await self.llm.chat(payload.get("prompt","Describe this image."), role="default", images=[img])
            return {"success":True,"description":desc}
        return {}

class MobileControlAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action=="execute_command":
            cmd=payload.get("command","")
            allowed=any(cmd.startswith(a) for a in CFG["allowed_commands"])
            if not allowed: return {"success":False,"error":"Command not allowed"}
            if "am start" in cmd:
                parts=cmd.split()
                for i,p in enumerate(parts):
                    if p=="-n" and i+1<len(parts):
                        pkg=parts[i+1].split("/")[0]
                        if pkg not in CFG["allowed_apps"]:
                            return {"success":False,"error":f"App {pkg} not allowed"}
            proc=await asyncio.create_subprocess_shell(cmd,stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
            o,e=await proc.communicate()
            return {"success":proc.returncode==0,"stdout":o.decode(errors="replace"),"stderr":e.decode(errors="replace")}
        elif action=="take_screenshot":
            path=SCREENSHOTS_DIR/f"shot_{int(time.time())}.png"
            proc=await asyncio.create_subprocess_shell(f"screencap {path}",stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
            await proc.communicate()
            return {"success":proc.returncode==0,"path":str(path)}
        return {}


# ══════════════════════════════════════════════════════════════════
#  🌐 BROWSER AGENT
# ══════════════════════════════════════════════════════════════════
class BrowserAgent(BaseAgent):
    """Full browser automation with Playwright (desktop) or lightweight scrape (mobile/Colab)."""

    async def _handle(self, action, payload):
        if action == "navigate":
            return await self._navigate(payload.get("url",""), payload.get("extract","text"))
        elif action == "click":
            return await self._playwright_action("click", payload)
        elif action == "fill_form":
            return await self._playwright_action("fill", payload)
        elif action == "screenshot":
            return await self._playwright_screenshot(payload.get("url",""))
        elif action == "search_web":
            query = urllib.parse.quote_plus(payload.get("query",""))
            return await self._navigate(f"https://duckduckgo.com/?q={query}&t=h_", "text")
        elif action == "shopping_compare":
            return await self._shopping_compare(payload.get("product",""))
        return {}

    async def _navigate(self, url, extract="text"):
        if HAS_PLAYWRIGHT and (IS_DESKTOP or IS_COLAB):
            return await self._playwright_navigate(url, extract)
        try:
            html = await _async_get_text(url, timeout=20)
            result = {"url": url, "raw_html": html[:5000]}
            if HAS_BS4:
                soup = BeautifulSoup(html, "html.parser")
                result["title"] = soup.title.string if soup.title else ""
                if extract == "text":
                    result["text"] = soup.get_text(separator=" ", strip=True)[:3000]
                elif extract == "links":
                    result["links"] = [{"href": a.get("href",""), "text": a.get_text(strip=True)}
                                       for a in soup.find_all("a", href=True)][:50]
            self.db.add_browser_visit(url, result.get("title",""))
            return {"success": True, **result}
        except Exception as e:
            return {"success": False, "error": str(e)}

    async def _playwright_navigate(self, url, extract="text"):
        try:
            async with async_playwright() as p:
                browser = await p.chromium.launch(headless=CFG["browser_headless"])
                page    = await browser.new_page()
                await page.goto(url, timeout=CFG["browser_timeout"], wait_until="domcontentloaded")
                title   = await page.title()
                self.db.add_browser_visit(url, title)
                result  = {"success":True, "url":url, "title":title}
                if extract == "text":
                    result["text"] = (await page.evaluate("document.body.innerText"))[:3000]
                elif extract == "screenshot":
                    spath = SCREENSHOTS_DIR / f"browser_{int(time.time())}.png"
                    await page.screenshot(path=str(spath))
                    result["screenshot"] = str(spath)
                elif extract == "links":
                    result["links"] = await page.evaluate(
                        "[...document.querySelectorAll('a')].slice(0,50).map(a=>({href:a.href,text:a.textContent.trim()}))"
                    )
                await browser.close()
                return result
        except Exception as e:
            return {"success":False,"error":str(e)}

    async def _playwright_action(self, action_type, payload):
        if not HAS_PLAYWRIGHT:
            return {"success":False,"error":"Playwright not installed."}
        url      = payload.get("url","")
        selector = payload.get("selector","")
        value    = payload.get("value","")
        try:
            async with async_playwright() as p:
                browser = await p.chromium.launch(headless=CFG["browser_headless"])
                page    = await browser.new_page()
                await page.goto(url, timeout=CFG["browser_timeout"])
                if action_type == "click":
                    await page.click(selector, timeout=5000)
                elif action_type == "fill":
                    await page.fill(selector, value, timeout=5000)
                spath = SCREENSHOTS_DIR / f"action_{int(time.time())}.png"
                await page.screenshot(path=str(spath))
                await browser.close()
                return {"success":True,"screenshot":str(spath)}
        except Exception as e:
            return {"success":False,"error":str(e)}

    async def _playwright_screenshot(self, url):
        return await self._playwright_navigate(url, extract="screenshot")

    async def _shopping_compare(self, product):
        stores = [
            f"https://www.amazon.com/s?k={urllib.parse.quote_plus(product)}",
            f"https://duckduckgo.com/?q={urllib.parse.quote_plus(product+' price buy')}",
        ]
        results = []
        for store_url in stores:
            r = await self._navigate(store_url, "text")
            if r.get("success"):
                text = r.get("text","")[:2000]
                analysis = await self.llm.chat(
                    f"Extract product names and prices from this shopping page text. Product: {product}\nText:\n{text}\nReturn JSON array: [{{name,price,currency,url}}]",
                    role="analyst"
                )
                try:
                    m = re.search(r"\[.*?\]", analysis, re.S)
                    if m: results.extend(json.loads(m.group()))
                except: pass
        return {"success":True,"product":product,"results":results}


# ══════════════════════════════════════════════════════════════════
#  👁️  OCR AGENT
# ══════════════════════════════════════════════════════════════════
class OCRAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action == "extract_text":
            return await self._ocr(payload.get("image_path",""), payload.get("lang","ara+eng"))
        elif action == "extract_from_screenshot":
            shot_path = SCREENSHOTS_DIR / f"ocr_{int(time.time())}.png"
            if IS_ANDROID:
                proc=await asyncio.create_subprocess_shell(f"screencap {shot_path}",stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
                await proc.communicate()
                return await self._ocr(str(shot_path), payload.get("lang","ara+eng"))
            return {"success":False,"error":"screenshot only on Android"}
        return {}

    async def _ocr(self, img_path, lang="ara+eng"):
        if not os.path.exists(img_path):
            return {"success":False,"error":"Image not found"}
        loop = asyncio.get_event_loop()
        try:
            result = await loop.run_in_executor(None, lambda: self._run_ocr(img_path, lang))
            return {"success":True,"text":result,"image":img_path}
        except Exception as e:
            return {"success":False,"error":str(e)}

    def _run_ocr(self, img_path, lang):
        if HAS_TESSERACT and HAS_PIL:
            img = Image.open(img_path)
            return pytesseract.image_to_string(img, lang=lang)
        result = subprocess.run(
            ["tesseract", img_path, "stdout", "-l", lang],
            capture_output=True, text=True, timeout=30
        )
        if result.returncode == 0:
            return result.stdout
        raise RuntimeError(f"tesseract error: {result.stderr[:200]}")


# ══════════════════════════════════════════════════════════════════
#  🗣️  VOICE AGENT  — [FIX] _play_audio empty-arg bug + Colab audio
# ══════════════════════════════════════════════════════════════════
class VoiceAgentCore(BaseAgent):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._whisper_model = None

    async def _handle(self, action, payload):
        if action == "speak":
            return await self._speak(payload.get("text",""), payload.get("lang", CFG["voice_language"]))
        elif action == "listen":
            return await self._listen(payload.get("duration_s", 5))
        elif action == "transcribe":
            return await self._transcribe(payload.get("audio_path",""))
        return {}

    async def _speak(self, text, lang="ar"):
        out_path = AUDIO_DIR / f"tts_{int(time.time())}.mp3"
        if HAS_EDGE_TTS:
            voice = CFG["tts_voice"] if lang=="ar" else CFG["tts_voice_en"]
            try:
                communicate = edge_tts.Communicate(text, voice)
                await communicate.save(str(out_path))
                await self._play_audio(str(out_path))
                return {"success":True,"method":"edge-tts","file":str(out_path)}
            except Exception as e:
                log.warning(f"edge-tts failed: {e}")
        if HAS_PYTTSX3 and (IS_DESKTOP or IS_WINDOWS or IS_MAC):
            loop = asyncio.get_event_loop()
            try:
                await loop.run_in_executor(None, lambda: self._pyttsx3_speak(text))
                return {"success":True,"method":"pyttsx3"}
            except Exception as e:
                log.warning(f"pyttsx3 failed: {e}")
        try:
            lang_code = "ar" if lang=="ar" else "en"
            proc=await asyncio.create_subprocess_exec(
                "espeak-ng", "-v", lang_code, text,
                stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE
            )
            await proc.communicate()
            return {"success":True,"method":"espeak-ng"}
        except Exception as e:
            return {"success":False,"error":f"All TTS methods failed: {e}"}

    def _pyttsx3_speak(self, text):
        engine = pyttsx3.init()
        engine.say(text)
        engine.runAndWait()

    async def _play_audio(self, path):
        # [FIX] removed empty-string argument; now builds args list properly
        players = ["mpv", "ffplay", "aplay", "mpg123"]
        for player in players:
            if shutil.which(player):
                args = [player, path]
                if player == "ffplay":
                    args += ["-nodisp", "-autoexit"]
                elif player == "mpv":
                    args += ["--no-video"]
                proc = await asyncio.create_subprocess_exec(
                    *args,
                    stdout=asyncio.subprocess.DEVNULL,
                    stderr=asyncio.subprocess.DEVNULL
                )
                await proc.communicate()
                return
        # Termux media player
        if IS_TERMUX:
            proc = await asyncio.create_subprocess_exec(
                "termux-media-player", "play", path,
                stdout=asyncio.subprocess.DEVNULL,
                stderr=asyncio.subprocess.DEVNULL
            )
            await proc.communicate()
            return
        # [NEW] Colab / Jupyter: use IPython Audio widget
        if IS_COLAB and HAS_IPYTHON_AUDIO and os.path.exists(path):
            try:
                _ipydisplay(_IPyAudio(path, autoplay=True))
            except Exception as e:
                log.warning(f"IPython audio failed: {e}")

    async def _listen(self, duration_s=5):
        wav_path = AUDIO_DIR / f"rec_{int(time.time())}.wav"
        recorder = None
        if shutil.which("arecord"):
            recorder = f"arecord -d {duration_s} -f cd -t wav {wav_path}"
        elif shutil.which("sox"):
            recorder = f"sox -d -d {wav_path} trim 0 {duration_s}"
        if recorder:
            proc=await asyncio.create_subprocess_shell(recorder,stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE)
            await proc.communicate()
            return await self._transcribe(str(wav_path))
        return {"success":False,"error":"No audio recorder found"}

    async def _transcribe(self, audio_path):
        if not os.path.exists(audio_path):
            return {"success":False,"error":"Audio file not found"}
        loop = asyncio.get_event_loop()
        if HAS_WHISPER:
            try:
                result = await loop.run_in_executor(None, lambda: self._whisper_transcribe(audio_path))
                return {"success":True,"text":result,"method":"faster-whisper"}
            except Exception as e: log.warning(f"faster-whisper failed: {e}")
        if shutil.which("whisper"):
            proc=await asyncio.create_subprocess_exec(
                "whisper","--model","base",audio_path,"--output_format","txt",
                stdout=asyncio.subprocess.PIPE,stderr=asyncio.subprocess.PIPE
            )
            out,_=await proc.communicate()
            if proc.returncode==0:
                return {"success":True,"text":out.decode(errors="replace").strip(),"method":"whisper-cli"}
        if HAS_SPEECH_RECOGNITION:
            try:
                result = await loop.run_in_executor(None, lambda: self._speech_recognition(audio_path))
                return {"success":True,"text":result,"method":"speech-recognition"}
            except Exception as e: return {"success":False,"error":str(e)}
        return {"success":False,"error":"No STT engine available."}

    def _whisper_transcribe(self, audio_path):
        if self._whisper_model is None:
            self._whisper_model = faster_whisper.WhisperModel(
                CFG["whisper_model_size"], device="cpu", compute_type="int8"
            )
        segments, _ = self._whisper_model.transcribe(audio_path, beam_size=5)
        return " ".join(s.text for s in segments).strip()

    def _speech_recognition(self, audio_path):
        recognizer = sr.Recognizer()
        with sr.AudioFile(audio_path) as source:
            audio = recognizer.record(source)
        lang = "ar-EG" if CFG["voice_language"]=="ar" else "en-US"
        return recognizer.recognize_google(audio, language=lang)


# ══════════════════════════════════════════════════════════════════
#  📅 CALENDAR AGENT
# ══════════════════════════════════════════════════════════════════
class CalendarAgent(BaseAgent):
    async def _handle(self, action, payload):
        if action == "add_event":
            return await self._add_event(payload)
        elif action == "list_events":
            return self._list_events(payload.get("days", 7))
        elif action == "parse_and_add":
            return await self._parse_natural(payload.get("text",""))
        elif action == "export_ics":
            return self._export_ics()
        return {}

    async def _add_event(self, data):
        title    = data.get("title","Untitled")
        start_dt = data.get("start_dt", datetime.now().isoformat())
        end_dt   = data.get("end_dt","")
        location = data.get("location","")
        desc     = data.get("description","")
        ics_file = str(CALENDAR_DIR / f"{uuid.uuid4().hex}.ics")
        ics_content = self._make_ics(title, start_dt, end_dt, location, desc)
        with open(ics_file,"w",encoding="utf-8") as f: f.write(ics_content)
        eid = self.db.add_calendar_event(title, start_dt, end_dt, location, desc, ics_file)
        return {"success":True,"event_id":eid,"ics_file":ics_file,"title":title,"start":start_dt}

    def _make_ics(self, title, start_dt, end_dt, location, desc):
        def fmt(dt_str):
            try:
                dt = datetime.fromisoformat(dt_str)
                return dt.strftime("%Y%m%dT%H%M%S")
            except: return datetime.now().strftime("%Y%m%dT%H%M%S")
        start = fmt(start_dt)
        end   = fmt(end_dt) if end_dt else start
        uid   = uuid.uuid4().hex + "@sps60"
        return (
            "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//SPS v60//EN\r\n"
            "BEGIN:VEVENT\r\n"
            f"UID:{uid}\r\n"
            f"DTSTART:{start}\r\n"
            f"DTEND:{end}\r\n"
            f"SUMMARY:{title}\r\n"
            f"LOCATION:{location}\r\n"
            f"DESCRIPTION:{desc}\r\n"
            "END:VEVENT\r\nEND:VCALENDAR\r\n"
        )

    def _list_events(self, days=7):
        events = self.db.get_upcoming_events(days)
        return {"success":True,"events":events,"count":len(events)}

    async def _parse_natural(self, text):
        resp = await self.llm.chat(
            f"Parse this event description and return JSON:\n{text}\n\n"
            'Return: {"title":str,"start_dt":"YYYY-MM-DDTHH:MM:SS","end_dt":"YYYY-MM-DDTHH:MM:SS","location":str,"description":str}',
            role="calendar"
        )
        try:
            m    = re.search(r"\{.*?\}",resp,re.S)
            data = json.loads(m.group()) if m else {}
            if "title" in data:
                return await self._add_event(data)
        except Exception as e: log.warning(f"Calendar parse error: {e}")
        return {"success":False,"error":"Could not parse event"}

    def _export_ics(self):
        all_ics = list(CALENDAR_DIR.glob("*.ics"))
        return {"success":True,"files":[str(f) for f in all_ics],"count":len(all_ics)}


# ══════════════════════════════════════════════════════════════════
#  📧 GMAIL AGENT
# ══════════════════════════════════════════════════════════════════
class GmailAgent(BaseAgent):
    async def _handle(self, action, payload):
        if not CFG["gmail_email"] or not CFG["gmail_password"]:
            return {"success":False,"error":"Set GMAIL_EMAIL and GMAIL_APP_PASSWORD env vars"}
        if action == "read_inbox":
            return await self._read_inbox(payload.get("limit",10))
        elif action == "send_email":
            return await self._send(payload.get("to",""), payload.get("subject",""), payload.get("body",""))
        elif action == "search":
            return await self._search(payload.get("query",""))
        return {}

    async def _read_inbox(self, limit=10):
        loop = asyncio.get_event_loop()
        try:
            messages = await loop.run_in_executor(None, lambda: self._imap_read(limit))
            return {"success":True,"messages":messages,"count":len(messages)}
        except Exception as e:
            return {"success":False,"error":str(e)}

    def _imap_read(self, limit):
        mail = imaplib.IMAP4_SSL("imap.gmail.com", 993)
        mail.login(CFG["gmail_email"], CFG["gmail_password"])
        mail.select("inbox")
        _, data = mail.search(None, "ALL")
        ids = data[0].split()[-limit:]
        messages = []
        for mid in reversed(ids):
            _, msg_data = mail.fetch(mid, "(RFC822)")
            msg = email.message_from_bytes(msg_data[0][1])
            subject = email.header.decode_header(msg["Subject"] or "")[0]
            subject_text = (subject[0].decode(subject[1] or "utf-8")
                            if isinstance(subject[0],bytes) else subject[0])
            from_addr = msg.get("From","")
            date      = msg.get("Date","")
            body = ""
            if msg.is_multipart():
                for part in msg.walk():
                    if part.get_content_type()=="text/plain":
                        body=part.get_payload(decode=True).decode(errors="replace")[:500]; break
            else:
                body=msg.get_payload(decode=True).decode(errors="replace")[:500]
            messages.append({"id":mid.decode(),"subject":subject_text,
                             "from":from_addr,"date":date,"body":body})
        mail.logout()
        return messages

    async def _send(self, to, subject, body):
        loop = asyncio.get_event_loop()
        try:
            await loop.run_in_executor(None, lambda: self._smtp_send(to, subject, body))
            return {"success":True,"to":to,"subject":subject}
        except Exception as e:
            return {"success":False,"error":str(e)}

    def _smtp_send(self, to, subject, body):
        msg = email.mime.multipart.MIMEMultipart()
        msg["From"]    = CFG["gmail_email"]
        msg["To"]      = to
        msg["Subject"] = subject
        msg.attach(email.mime.text.MIMEText(body, "plain", "utf-8"))
        with smtplib.SMTP_SSL("smtp.gmail.com", 465) as smtp:
            smtp.login(CFG["gmail_email"], CFG["gmail_password"])
            smtp.send_message(msg)

    async def _search(self, query):
        loop = asyncio.get_event_loop()
        try:
            results = await loop.run_in_executor(None, lambda: self._imap_search(query))
            return {"success":True,"results":results}
        except Exception as e:
            return {"success":False,"error":str(e)}

    def _imap_search(self, query):
        mail = imaplib.IMAP4_SSL("imap.gmail.com",993)
        mail.login(CFG["gmail_email"],CFG["gmail_password"])
        mail.select("inbox")
        _,data = mail.search(None,f'SUBJECT "{query}"')
        ids = data[0].split()[-10:]
        results = []
        for mid in ids:
            results.append({"id":mid.decode()})
        mail.logout()
        return results


# ══════════════════════════════════════════════════════════════════
#  AGENT CLASSES REGISTRY
# ══════════════════════════════════════════════════════════════════
AGENT_CLASSES = {
    "planner":  PlannerAgent,
    "coder":    CoderAgent,
    "reviewer": ReviewerAgent,
    "debugger": DebuggerAgent,
    "vision":   VisionAgent,
    "mobile":   MobileControlAgent,
    "browser":  BrowserAgent,
    "ocr":      OCRAgent,
    "voice":    VoiceAgentCore,
    "calendar": CalendarAgent,
    "gmail":    GmailAgent,
}

# [NEW] Agents skipped in low-memory mode (heavy resource consumers)
LOW_MEMORY_SKIP_AGENTS = {"voice", "ocr", "vision"} if IS_LOW_MEMORY else set()


# ══════════════════════════════════════════════════════════════════
#  🔄 SELF-REPAIR AGENT
# ══════════════════════════════════════════════════════════════════
class SelfRepairAgent(BaseAgent):
    def __init__(self, *args, orch=None, **kwargs):
        super().__init__(*args, **kwargs)
        self.orch = orch

    async def _handle(self, action, payload):
        if action == "repair":
            return await self._repair(
                payload.get("task",""),
                payload.get("code",""),
                payload.get("error",""),
                payload.get("agent_role",""),
            )
        return {}

    async def _repair(self, task, code, error, agent_role):
        log.info(f"🔄 SelfRepair: {agent_role} | error: {error[:80]}")
        best_code = code
        fixed = False
        for attempt in range(CFG["self_repair_max_attempts"]):
            known = self._lookup_known_fix(error)
            if known:
                prompt = f"Apply this known fix strategy: {known}\nTo fix error: {error}\nIn code:\n{best_code[:1500]}"
            else:
                prompt = (
                    f"Fix this Python code that has the following error.\n"
                    f"Error: {error}\n"
                    f"Task: {task}\n"
                    f"Code:\n{best_code[:1500]}\n\n"
                    f"Attempt {attempt+1}. Output only corrected Python code."
                )
            fixed_code = await self.llm.chat(prompt, role="repair")
            fixed_code = re.sub(r"```python|```","",fixed_code).strip()
            ok, out, _, _ = await self.sandbox.run(fixed_code, timeout=15)
            if ok:
                eval_resp = await self.llm.chat(
                    f"Task: {task}\nOutput: {out[:300]}\nDoes this output indicate the code works? YES or NO",
                    role="analyst"
                )
                if "YES" in eval_resp.upper():
                    best_code = fixed_code
                    fixed = True
                    if agent_role and self.orch:
                        self.db.save_agent(agent_role, best_code)
                        log.info(f"✅ SelfRepair: {agent_role} patched!")
                    sig = hashlib.md5(error[:100].encode()).hexdigest()
                    self.db.add_failure(sig, error[:200], f"Fixed on attempt {attempt+1}")
                    break
            best_code = fixed_code
        self.db.add_repair(task, error, attempt+1, fixed)
        return {"success":fixed,"code":best_code,"attempts":attempt+1}

    def _lookup_known_fix(self, error):
        rows = self._db_fetch_fix(error[:100])
        return rows[0]["fix_strategy"] if rows else None

    def _db_fetch_fix(self, sig_prefix):
        try:
            return [dict(r) for r in self.db.conn.execute(
                "SELECT fix_strategy FROM failure_patterns WHERE error_signature LIKE ? ORDER BY occurrence DESC LIMIT 1",
                (f"%{sig_prefix[:50]}%",)
            ).fetchall()]
        except: return []


# ══════════════════════════════════════════════════════════════════
#  🎯 LONG-TERM PLANNER
# ══════════════════════════════════════════════════════════════════
class LongTermPlanner:
    def __init__(self, orch):
        self.orch = orch

    async def create_plan(self, goal: str, max_steps: int = None) -> Dict:
        max_steps = max_steps or CFG["max_plan_steps"]
        log.info(f"🎯 LongTermPlanner: creating plan for '{goal[:60]}'")
        resp = await self.orch.llm.chat(
            f"Break down this complex goal into {max_steps} concrete steps.\n"
            f"Goal: {goal}\n\n"
            f"Output JSON array: [{{'step':int,'task':str,'depends_on':int_or_null,"
            f"'estimated_duration':'Xmin','agent_type':str}}]\n"
            f"agent_type must be one of: coder|browser|vision|ocr|voice|calendar|gmail|planner\n"
            f"Be specific and actionable. Max {max_steps} steps.",
            role="planner"
        )
        try:
            m = re.search(r"\[.*\]", resp, re.S)
            steps = json.loads(m.group()) if m else [{"step":1,"task":goal,"agent_type":"coder"}]
        except:
            steps = [{"step":1,"task":goal,"agent_type":"coder"}]
        plan_id = uuid.uuid4().hex
        self.orch.db.save_plan(plan_id, goal, steps, current=0, status="active")
        log.info(f"✅ Plan {plan_id[:8]} created: {len(steps)} steps")
        return {"plan_id":plan_id, "goal":goal, "steps":steps, "total":len(steps)}

    async def execute_plan(self, plan_id: str) -> Dict:
        plan = self.orch.db.get_plan(plan_id)
        if not plan:
            return {"success":False,"error":"Plan not found"}
        steps        = plan["steps"]
        current_step = plan["current_step"]
        results      = {}
        log.info(f"▶️  Executing plan {plan_id[:8]} from step {current_step+1}/{len(steps)}")
        for i, step in enumerate(steps[current_step:], start=current_step):
            step_num   = step.get("step", i+1)
            task       = step.get("task","")
            agent_type = step.get("agent_type","coder")
            if not ResourceMonitor.safe():
                log.warning("⚠️  Resources low — pausing plan execution")
                self.orch.db.update_plan_step(plan_id, i, "paused")
                return {"success":False,"error":"Resource limit","paused_at":i,"results":results}
            log.info(f"  ▸ Step {step_num}/{len(steps)}: {task[:50]}")
            result = await self._execute_step(task, agent_type)
            results[f"step_{step_num}"] = result
            if i % CFG["plan_checkpoint_interval"] == 0:
                self.orch.db.update_plan_step(plan_id, i+1, "active")
            if not result.get("success"):
                log.warning(f"  ⚠️  Step {step_num} failed. Attempting repair...")
                repair_agent = self.orch.agent_mgr.pools.get("repair",[None])[0]
                if repair_agent:
                    repair_resp = await self.orch.bus.request(
                        repair_agent.id, "repair",
                        {"task":task,"code":result.get("code",""),
                         "error":result.get("error",""),"agent_role":agent_type}
                    )
                    if repair_resp and repair_resp.get("success"):
                        results[f"step_{step_num}"] = {"success":True,"output":"Repaired & re-executed","repaired":True}
                        continue
                if step.get("critical", True):
                    self.orch.db.update_plan_step(plan_id, i, "failed")
                    return {"success":False,"failed_at":step_num,"results":results}
        self.orch.db.update_plan_step(plan_id, len(steps), "completed")
        return {"success":True,"plan_id":plan_id,"steps_completed":len(steps),"results":results}

    async def _execute_step(self, task: str, agent_type: str) -> Dict:
        pools = self.orch.agent_mgr.pools
        agent_map = {
            "coder":    ("coder",    "write_code",   {"task":task,"style":"simple"}),
            "browser":  ("browser",  "search_web",   {"query":task}),
            "vision":   ("vision",   "analyze_image",{"image_path":"","prompt":task}),
            "ocr":      ("ocr",      "extract_text", {"image_path":"","lang":"ara+eng"}),
            "voice":    ("voice",    "speak",        {"text":task}),
            "calendar": ("calendar", "parse_and_add",{"text":task}),
            "gmail":    ("gmail",    "read_inbox",   {"limit":5}),
            "planner":  ("planner",  "plan",         {"goal":task}),
        }
        mapping    = agent_map.get(agent_type, agent_map["coder"])
        pool_name, action, payload = mapping
        agent_list = pools.get(pool_name, [])
        if not agent_list:
            return await self.orch.execute(task)
        result = await self.orch.bus.request(agent_list[0].id, action, payload)
        if result is None:
            return {"success":False,"error":f"Agent {pool_name} did not respond"}
        if agent_type=="coder" and "code" in result:
            ok, out, _, _ = await self.orch.sandbox.run(result["code"])
            return {"success":ok,"output":out,"code":result["code"]}
        return {"success":True,**result}

    async def resume_active_plans(self):
        plans = self.orch.db.get_active_plans()
        if plans:
            log.info(f"📋 Resuming {len(plans)} active plan(s)")
            for plan in plans:
                asyncio.create_task(self.execute_plan(plan["id"]))


# ══════════════════════════════════════════════════════════════════
#  ISOLATED AGENT PROXY + FACTORY
# ══════════════════════════════════════════════════════════════════
class IsolatedAgentProxy(BaseAgent):
    def __init__(self, aid, role, code, llm, db, bus, sandbox):
        super().__init__(aid, role, llm, bus, sandbox, db)
        self.code = code

    async def _handle(self, action, payload):
        if action == "execute":
            task = payload.get("task","")
            ok, out, q, _ = await self.sandbox.run(f"__task__={json.dumps(task)}\n{self.code}")
            return {"success":ok,"output":out,"quality":q}
        return {}


class SecureAgentFactory:
    def __init__(self, llm, db, bus, sandbox):
        self.llm=llm; self.db=db; self.bus=bus; self.sandbox=sandbox; self.created=0

    async def create_agent(self, desc):
        if self.created >= CFG["max_agents"]: return None
        code = await self.llm.chat(
            f"Create Python class CustomAgent inheriting BaseAgent. Override _handle for: {desc}. Raw code only.",
            role="coder"
        )
        code = re.sub(r"```python|```","",code).strip()
        try: ast.parse(code)
        except: return None
        test = (f"import asyncio\n{code}\n"
                "async def test():\n    a=CustomAgent('t','r',None,None,None,None)\n    print('OK')\n"
                "asyncio.run(test())")
        ok,out,_,_ = await self.sandbox.run(test, timeout=10)
        if not ok or "OK" not in out: return None
        role = f"custom_{self.created}"
        self.db.save_agent(role, code)
        a = IsolatedAgentProxy(f"{role}_0",role,code,self.llm,self.db,self.bus,self.sandbox)
        await a.start(); self.created+=1
        log.info(f"✅ Created custom agent: {role}")
        return a


# ══════════════════════════════════════════════════════════════════
#  AGENT MANAGER
# ══════════════════════════════════════════════════════════════════
class AgentManager:
    def __init__(self, llm, db, bus, sandbox, factory):
        self.llm=llm; self.db=db; self.bus=bus; self.sandbox=sandbox; self.factory=factory
        self.pools: Dict[str,List] = defaultdict(list)

    async def create_agent(self, desc):
        a = await self.factory.create_agent(desc)
        if a: self.pools[a.role].append(a)
        return a

    async def start_watchdog(self): pass

    async def shutdown(self):
        for agents in self.pools.values():
            for a in agents: await a.stop()


# ══════════════════════════════════════════════════════════════════
#  RL SCORER
# ══════════════════════════════════════════════════════════════════
class SimpleRLScorer:
    def __init__(self):
        self.actions = ["simple","oop","functional"]
        self.scores  = {a:{"total":0.0,"count":0} for a in self.actions}
        self.epsilon = CFG["rl_epsilon"]

    def choose_action(self):
        if random.random() < self.epsilon: return random.choice(self.actions)
        return max(self.actions, key=lambda a: self.scores[a]["total"]/(self.scores[a]["count"]+1))

    def update(self, action, reward):
        if action in self.scores:
            self.scores[action]["total"] += reward
            self.scores[action]["count"] += 1


# ══════════════════════════════════════════════════════════════════
#  VIDEO MAKER + APP BUILDER
# ══════════════════════════════════════════════════════════════════
class VideoMakerAgent:
    def __init__(self, llm, tool_mgr):
        self.llm      = llm
        self.tool_mgr = tool_mgr

    async def create_short(self, prompt):
        out = PROJECTS_DIR / f"{uuid.uuid4().hex}.mp4"
        proc = await asyncio.create_subprocess_shell(
            f"ffmpeg -f lavfi -i color=c=black:s=1280x720:d=5 -an {out} -y",
            stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE
        )
        await proc.communicate()
        return str(out)


class AppBuilderAgent:
    def __init__(self, tool_mgr):
        self.tool_mgr = tool_mgr

    async def build_web_app(self, spec):
        proj = PROJECTS_DIR / f"web_{uuid.uuid4().hex[:8]}"
        proj.mkdir()
        (proj/"index.html").write_text(
            f"<!DOCTYPE html><html><body><h1>{spec}</h1></body></html>", encoding="utf-8"
        )
        return proj


# ══════════════════════════════════════════════════════════════════
#  AGENT ARCHITECT
# ══════════════════════════════════════════════════════════════════
class AgentArchitect:
    def __init__(self, orch): self.orch = orch

    async def build_agent_from_idea(self, idea):
        analysis_resp = await self.orch.llm.chat(
            f'Analyze: {idea}\nReturn JSON: {{"goal":str,"skills":[str],"tools":[str]}}', role="planner")
        try:
            m = re.search(r"\{.*?\}",analysis_resp,re.S)
            analysis = json.loads(m.group()) if m else {"goal":idea}
        except: analysis = {"goal":idea}

        design_resp = await self.orch.llm.chat(
            f'Design agent for: {json.dumps(analysis)}\nReturn JSON: {{"architecture":str,"sub_agents":[str],"description":str}}',
            role="planner")
        try:
            m = re.search(r"\{.*?\}",design_resp,re.S)
            design = json.loads(m.group()) if m else {"architecture":"single"}
        except: design = {"architecture":"single"}

        coder = self.orch.agent_mgr.pools["coder"][0]
        code_resp = await self.orch.bus.request(
            coder.id,"write_code",{"task":f"Create agent for: {json.dumps(design)}","style":"oop"})
        if not code_resp: return {"success":False,"error":"Coder failed"}
        code = code_resp.get("code","")
        ok,out,_,_ = await self.orch.sandbox.run(code, timeout=CFG["sandbox_timeout"])
        if not ok:
            dbg = self.orch.agent_mgr.pools["debugger"][0]
            fix = await self.orch.bus.request(dbg.id,"debug",{"code":code,"error":out})
            if fix: code = fix.get("fix",code)
        agent = await self.orch.agent_mgr.create_agent(f"Auto-built: {idea}")
        if not agent: return {"success":False,"error":"Instantiation failed"}
        return {"success":True,"analysis":analysis,"design":design,"agent_role":agent.role}


# ══════════════════════════════════════════════════════════════════
#  EXECUTION ENGINE + CONTEXT PLANNER + FAILURE ANALYZER
# ══════════════════════════════════════════════════════════════════
class ExecutionEngine:
    def __init__(self, orch): self.orch = orch

    async def execute_goal(self, goal):
        plan    = await self.orch.ctx_planner.plan(goal)
        results = {}
        for step in plan:
            task   = step.get("task", goal)
            agents = self.orch.agent_mgr.pools.get("coder",[])
            if not agents: return {"success":False,"error":"No coder"}
            cr = await self.orch.bus.request(agents[0].id,"write_code",
                                              {"task":task,"style":self.orch.code_style})
            if not cr: continue
            ok,out,_,_ = await self.orch.sandbox.run(cr.get("code",""))
            results[f"step_{step.get('step',1)}"] = {"success":ok,"output":out}
            if not ok: break
        return {
            "success": all(r["success"] for r in results.values()) if results else False,
            "steps":   results,
            "output":  list(results.values())[-1].get("output","") if results else "",
        }


class ContextPlanner:
    def __init__(self, orch): self.orch = orch

    async def plan(self, goal):
        resp = await self.orch.llm.chat(
            f"Create steps to accomplish: {goal}\nOutput JSON array: [{{\"step\":int,\"task\":str}}]",
            role="planner")
        try:
            m = re.search(r"\[.*?\]",resp,re.S)
            return json.loads(m.group()) if m else [{"step":1,"task":goal}]
        except: return [{"step":1,"task":goal}]


class FailureAnalyzer:
    def __init__(self, llm, db): self.llm=llm; self.db=db

    async def analyze(self, task, code, error):
        resp = await self.llm.chat(f"Root-cause: {error}\nTask: {task}", role="analyst")
        try:
            m = re.search(r"\{.*?\}",resp,re.S)
            return json.loads(m.group()) if m else {}
        except: return {}


class MetaCognition:
    def __init__(self, orch): self.orch = orch

    async def run_cycle(self):
        metrics = self.orch.db.recent_metrics(20)
        if not metrics: return
        sr = sum(m["success_rate"] for m in metrics)/len(metrics)
        if sr < 0.4:
            await self.orch.agent_mgr.create_agent("versatile helper")
        self.orch.db.add_metric(sr, 0.0, len(self.orch.agent_mgr.pools))


class StrategicGoalEngine:
    def __init__(self, db, llm, vs): self.db=db; self.llm=llm; self.vs=vs

    async def run_cycle(self, orch):
        if not ResourceMonitor.safe(): return
        resp = await self.llm.chat("Generate 3 strategic improvement goals as JSON array.", role="planner")
        try:
            m     = re.search(r"\[.*?\]",resp,re.S)
            goals = json.loads(m.group()) if m else ["improve reliability"]
        except: goals = ["improve reliability"]
        for g in goals[:2]: self.db.add_goal(str(g), priority=1)


# ══════════════════════════════════════════════════════════════════
#  TELEGRAM BOT
# ══════════════════════════════════════════════════════════════════
class TelegramBot:
    def __init__(self, orch): self.orch=orch; self.token=CFG["telegram_token"]; self.app=None

    async def start(self):
        if not self.token or not HAS_TELEGRAM: return
        self.app = Application.builder().token(self.token).build()
        self.app.add_handler(CommandHandler("start",   self._start_cmd))
        self.app.add_handler(CommandHandler("run",     self._run_cmd))
        self.app.add_handler(CommandHandler("plan",    self._plan_cmd))
        self.app.add_handler(CommandHandler("status",  self._status_cmd))
        self.app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, self._handle))
        await self.app.initialize(); await self.app.start(); await self.app.updater.start_polling()
        log.info("✅ Telegram polling started")

    async def stop(self):
        if self.app:
            try:
                await self.app.updater.stop(); await self.app.stop(); await self.app.shutdown()
            except: pass

    async def _start_cmd(self, u, c):
        await u.message.reply_text("🤖 SPS v60.2 DOMINATOR ready!\n/run <task>\n/plan <complex goal>\n/status")

    async def _run_cmd(self, u, c):
        task = " ".join(c.args) if c.args else ""
        if not task: await u.message.reply_text("Usage: /run <task>"); return
        await u.message.reply_text(f"⏳ {task[:50]}…")
        res = await self.orch.execute(task)
        if res.get("success"):
            await u.message.reply_text(f"✅ {str(res.get('output',''))[:500]}")
        else:
            await u.message.reply_text(f"❌ {res.get('error','')}")

    async def _plan_cmd(self, u, c):
        goal = " ".join(c.args) if c.args else ""
        if not goal: await u.message.reply_text("Usage: /plan <complex goal>"); return
        await u.message.reply_text("🎯 Creating long-term plan…")
        plan = await self.orch.long_term_planner.create_plan(goal)
        await u.message.reply_text(f"✅ Plan created: {plan['total']} steps\nID: {plan['plan_id'][:8]}")
        asyncio.create_task(self.orch.long_term_planner.execute_plan(plan["plan_id"]))

    async def _status_cmd(self, u, c):
        s = self.orch.status()
        await u.message.reply_text(f"📊 {json.dumps(s, indent=2)[:400]}")

    async def _handle(self, u, c):
        await self._run_cmd(u, c)


# ══════════════════════════════════════════════════════════════════
#  SCHEDULER
# ══════════════════════════════════════════════════════════════════
class Scheduler:
    def __init__(self, orch): self.orch=orch; self._task=None

    async def start(self): self._task = asyncio.create_task(self._loop())

    async def _loop(self):
        while True:
            await asyncio.sleep(CFG["schedule_check_interval"])
            if not ResourceMonitor.safe(): continue
            for s in self.orch.db.get_due_schedules():
                asyncio.create_task(self.orch.execute(s["task_desc"]))
                self.orch.db.update_schedule_last_run(s["id"])
            self.orch.db.purge_browser_history()
            StealthMode.run_cycle()

    async def stop(self):
        if self._task: self._task.cancel()


# ══════════════════════════════════════════════════════════════════
#  🤖 AGENT OS — Main Orchestrator  (all fixes applied)
# ══════════════════════════════════════════════════════════════════
class AgentOS:
    ROUTE_MAP = {
        "browser": ["browse","visit","scrape","website","webpage","url","http","search online","google","open site"],
        "ocr":     ["ocr","read image text","extract text from","read screenshot","text in image"],
        "voice":   ["speak","say","read aloud","voice","tts","listen","transcribe","hear","audio"],
        "calendar":["schedule","calendar","appointment","event","remind me","meeting at","book time"],
        "gmail":   ["email","gmail","send mail","inbox","check email","read mail"],
        "video":   ["video","create video","make video","record"],
        "plan":    ["plan","long-term","multi-step","complex task","over days","multiple steps"],
        "app":     ["build app","create app","web app","android app"],
        "agent":   ["build agent","create agent","new agent","spawn agent"],
        "vision":  ["analyze image","describe image","what is in","look at image"],
        "mobile":  ["open app","mobile","android","launch","tap","swipe"],
    }

    def __init__(self):
        self.db             = DB()
        self.llm            = LLM(self.db)
        self.vs             = FAISSVectorMemory(self.llm)
        self.bus            = SwarmBus()
        self.sandbox        = SecureSandbox()
        self.factory        = SecureAgentFactory(self.llm, self.db, self.bus, self.sandbox)
        self.agent_mgr      = AgentManager(self.llm, self.db, self.bus, self.sandbox, self.factory)
        self.ctx_planner    = ContextPlanner(self)
        self.fail_ana       = FailureAnalyzer(self.llm, self.db)
        self.meta           = MetaCognition(self)
        self.goal_engine    = StrategicGoalEngine(self.db, self.llm, self.vs)
        self.exec_engine    = ExecutionEngine(self)
        # [FIX] tool_mgr removed — VideoMakerAgent/AppBuilderAgent use None safely
        self.video_maker    = VideoMakerAgent(self.llm, None)
        self.app_builder    = AppBuilderAgent(None)
        self.architect      = AgentArchitect(self)
        self.long_term_planner = LongTermPlanner(self)
        self.telegram       = TelegramBot(self)
        self.scheduler      = Scheduler(self)
        self.scorer         = SimpleRLScorer()
        self.code_style     = "simple"
        self._tasks: List[asyncio.Task] = []
        self.update_url     = CFG["update_url"]

    async def initialize(self, skip_heavy: bool = False):
        log.info(f"🚀 SPS v60.2 DOMINATOR starting on {PLATFORM_TAG} "
                 f"({'low-mem' if IS_LOW_MEMORY else 'normal'})")
        await self.llm.check()

        # [NEW] Determine which agents to skip on weak/low-memory devices
        skip_roles = LOW_MEMORY_SKIP_AGENTS if (IS_LOW_MEMORY or skip_heavy) else set()
        if skip_roles:
            log.info(f"📱 Skipping heavy agents: {skip_roles}")

        for role, cls in AGENT_CLASSES.items():
            if role in skip_roles:
                continue
            try:
                a = cls(f"{role}_0", role, self.llm, self.bus, self.sandbox, self.db)
            except Exception as e:
                log.warning(f"Could not create agent {role}: {e}")
                continue
            await a.start()
            self.agent_mgr.pools[role].append(a)

        # SelfRepairAgent (always needed)
        repair_a = SelfRepairAgent(f"repair_0","repair",self.llm,self.bus,self.sandbox,self.db,orch=self)
        await repair_a.start()
        self.agent_mgr.pools["repair"].append(repair_a)

        # Load persisted custom agents
        for rec in self.db.list_agents():
            if rec["role"] not in self.agent_mgr.pools:
                code = self.db.get_agent_code(rec["role"])
                if code:
                    a = IsolatedAgentProxy(f"{rec['role']}_0",rec["role"],code,
                                           self.llm,self.db,self.bus,self.sandbox)
                    await a.start()
                    self.agent_mgr.pools[rec["role"]].append(a)

        await self.telegram.start()
        await self.scheduler.start()
        self._tasks += [
            asyncio.create_task(self._meta_loop()),
            asyncio.create_task(self._goal_loop()),
        ]
        await self.long_term_planner.resume_active_plans()

        log.info("═══════════════════════════════════════════════")
        log.info(f"  SPS v60.2 DOMINATOR — READY")
        log.info(f"  Platform   : {PLATFORM_TAG}")
        log.info(f"  Low-memory : {IS_LOW_MEMORY}")
        log.info(f"  Colab      : {IS_COLAB}")
        log.info(f"  Agents     : {list(self.agent_mgr.pools.keys())}")
        log.info(f"  Groq       : {'✅' if self.llm._has('groq_api_key') else '❌'}")
        log.info(f"  Ollama     : {'✅' if self.llm._ollama_alive else '?'}")
        log.info("═══════════════════════════════════════════════")

    async def _meta_loop(self):
        while True:
            await asyncio.sleep(CFG["meta_cog_interval"])
            await self.meta.run_cycle()

    async def _goal_loop(self):
        while True:
            await asyncio.sleep(CFG["goal_generation_interval"])
            await self.goal_engine.run_cycle(self)

    def _route(self, task_desc: str) -> str:
        td = task_desc.lower()
        for route, keywords in self.ROUTE_MAP.items():
            if any(k in td for k in keywords):
                return route
        return "execute"

    async def execute(self, task_desc: str) -> Dict:
        if not self.agent_mgr.pools:
            return {"success":False,"error":"No agents initialized"}
        if not ResourceMonitor.safe():
            return {"success":False,"error":"Resource limit exceeded"}

        route = self._route(task_desc)
        td    = task_desc.lower()

        if route == "plan":
            plan = await self.long_term_planner.create_plan(task_desc)
            asyncio.create_task(self.long_term_planner.execute_plan(plan["plan_id"]))
            return {"success":True,
                    "output":f"Plan created: {plan['total']} steps. Running in background.",
                    "plan_id":plan["plan_id"]}

        if route == "agent":
            return await self.architect.build_agent_from_idea(task_desc)

        if route == "video":
            out = await self.video_maker.create_short(task_desc)
            return {"success":True,"output":out}

        if route == "app":
            proj = await self.app_builder.build_web_app(task_desc)
            return {"success":True,"output":str(proj)}

        if route == "browser":
            agent = self.agent_mgr.pools.get("browser",[None])[0]
            if agent:
                url_match = re.search(r"https?://\S+", task_desc)
                if url_match:
                    return await self.bus.request(agent.id,"navigate",
                                                  {"url":url_match.group(),"extract":"text"}) or {"success":False,"error":"Timeout"}
                return await self.bus.request(agent.id,"search_web",{"query":task_desc}) or {"success":False,"error":"Timeout"}

        if route == "ocr":
            agent = self.agent_mgr.pools.get("ocr",[None])[0]
            if agent:
                for word in task_desc.split():
                    if os.path.exists(word):
                        return await self.bus.request(agent.id,"extract_text",{"image_path":word}) or {}
                return {"success":False,"error":"No image path found in task"}

        if route == "voice":
            agent = self.agent_mgr.pools.get("voice",[None])[0]
            if agent:
                if any(k in td for k in ["speak","say","read aloud","tts"]):
                    text = re.sub(r"(speak|say|read aloud|tts)\s*:?\s*","",task_desc,flags=re.I).strip()
                    return await self.bus.request(agent.id,"speak",{"text":text}) or {}
                elif any(k in td for k in ["listen","transcribe"]):
                    return await self.bus.request(agent.id,"listen",{"duration_s":5}) or {}

        if route == "calendar":
            agent = self.agent_mgr.pools.get("calendar",[None])[0]
            if agent:
                if "list" in td or "show" in td or "upcoming" in td:
                    return await self.bus.request(agent.id,"list_events",{"days":7}) or {}
                return await self.bus.request(agent.id,"parse_and_add",{"text":task_desc}) or {}

        if route == "gmail":
            agent = self.agent_mgr.pools.get("gmail",[None])[0]
            if agent:
                if any(k in td for k in ["send","write email","compose"]):
                    parsed = await self.llm.chat(
                        f"Extract: to_email, subject, body from: {task_desc}\nReturn JSON.",
                        role="analyst"
                    )
                    try:
                        m = re.search(r"\{.*?\}",parsed,re.S)
                        params = json.loads(m.group()) if m else {}
                        return await self.bus.request(agent.id,"send_email",params) or {}
                    except: return {"success":False,"error":"Could not parse email params"}
                return await self.bus.request(agent.id,"read_inbox",{"limit":10}) or {}

        if route == "vision":
            agent = self.agent_mgr.pools.get("vision",[None])[0]
            if agent:
                for word in task_desc.split():
                    if os.path.exists(word):
                        return await self.bus.request(agent.id,"analyze_image",
                                                      {"image_path":word,"prompt":task_desc}) or {}
                return {"success":False,"error":"No image path found"}

        if route == "mobile":
            agent = self.agent_mgr.pools.get("mobile",[None])[0]
            if agent:
                cmd = task_desc
                if "open " in td:
                    app_map = {
                        "chrome":   "com.android.chrome",
                        "termux":   "com.termux",
                        "telegram": "org.telegram.messenger",
                        "youtube":  "com.google.android.youtube",
                    }
                    app = task_desc.split("open ")[-1].strip().lower()
                    pkg = app_map.get(app, app)
                    cmd = f"am start -n {pkg}/.MainActivity"
                return await self.bus.request(agent.id,"execute_command",{"command":cmd}) or {}

        # ── Default: code execution path ──────────────────────────
        self.code_style = self.scorer.choose_action()
        result  = await self.exec_engine.execute_goal(task_desc)
        reward  = 1.0 if result["success"] else -0.5
        self.scorer.update(self.code_style, reward)
        tid = uuid.uuid4().hex
        self.db.save_task(tid, task_desc, "done" if result["success"] else "failed",
                          result=result, reward=reward)
        if result["success"]:
            key = hashlib.md5(task_desc.encode()).hexdigest()
            self.db.add_memory(key, result.get("output",""), task_desc, reward)
            asyncio.create_task(self.vs.add(task_desc, result.get("output",""), reward))
        if not result["success"] and CFG["self_evolve"]:
            repair_agent = self.agent_mgr.pools.get("repair",[None])[0]
            if repair_agent:
                asyncio.create_task(
                    self.bus.request(repair_agent.id,"repair",
                                     {"task":task_desc,"code":"",
                                      "error":result.get("error",""),"agent_role":"coder"})
                )
        return result

    # ── Self-update  [FIX] use global BACKUP_DIR ──────────────────
    async def self_update(self, update_url=None):
        url = update_url or self.update_url
        BACKUP_DIR.mkdir(exist_ok=True)           # [FIX] use global, not local shadow
        current_file = Path(__file__)
        try:
            log.info(f"🔄 Downloading update from: {url}")
            if HAS_AIOHTTP:
                async with aiohttp.ClientSession() as s:
                    async with s.get(url, timeout=aiohttp.ClientTimeout(total=30)) as r:
                        if r.status != 200: return f"❌ HTTP {r.status}"
                        new_code = await r.text()
            else:
                loop = asyncio.get_event_loop()
                def _sync():
                    req = urllib.request.Request(url)
                    with urllib.request.urlopen(req, timeout=30) as resp:
                        return resp.read().decode()
                new_code = await loop.run_in_executor(None, _sync)

            if not new_code:
                return "❌ Failed to download update."

            timestamp   = datetime.now().strftime("%Y%m%d_%H%M%S")
            backup_path = BACKUP_DIR / f"sps_{timestamp}.py"
            shutil.copy2(current_file, backup_path)
            log.info(f"📦 Backup saved: {backup_path.name}")

            try:
                ast.parse(new_code)
            except SyntaxError as e:
                log.error(f"❌ New code has syntax errors: {e}")
                return f"❌ Update failed: Syntax error in new code. Original preserved."

            with open(current_file, "w", encoding="utf-8") as f:
                f.write(new_code)

            shutil.copy2(current_file, BACKUP_DIR / "sps_latest_stable.py")
            log.info("✅ Update successful! Restarting...")
            await asyncio.sleep(1)
            os.execv(sys.executable, [sys.executable] + sys.argv)
        except Exception as e:
            log.exception("Update failed")
            return f"❌ Update error: {e}"

    # ── Rollback  [FIX] use global BACKUP_DIR ─────────────────────
    async def rollback(self):
        stable = BACKUP_DIR / "sps_latest_stable.py"  # [FIX] use global
        if not stable.exists():
            return "❌ No stable backup found."
        try:
            shutil.copy2(stable, __file__)
            log.info("✅ Rolled back to last stable version. Restarting...")
            await asyncio.sleep(1)
            os.execv(sys.executable, [sys.executable] + sys.argv)
        except Exception as e:
            return f"❌ Rollback failed: {e}"

    # ── Status  [FIX] _ollama_alive None → False ──────────────────
    def status(self) -> Dict:
        return {
            "version":   "v60.2-DOMINATOR",
            "platform":  PLATFORM_TAG,
            "low_memory":IS_LOW_MEMORY,
            "colab":     IS_COLAB,
            "agents":    {r:len(v) for r,v in self.agent_mgr.pools.items()},
            "resources": ResourceMonitor.stats(),
            "llm": {
                "groq":   self.llm._has("groq_api_key"),
                "openai": self.llm._has("openai_api_key"),
                "ollama": bool(self.llm._ollama_alive),   # [FIX] None → False
            },
        }

    async def shutdown(self):
        for t in self._tasks: t.cancel()
        await self.agent_mgr.shutdown()
        await self.telegram.stop()
        await self.scheduler.stop()
        StealthMode.run_cycle()
        log.info("SPS v60.2 shutdown complete.")


# ══════════════════════════════════════════════════════════════════
#  INTERACTIVE LOOP
# ══════════════════════════════════════════════════════════════════
HELP_TEXT = """
╔══════════════════════════════════════════════════════════╗
║  SPS v60.2 DOMINATOR — Commands                          ║
╠══════════════════════════════════════════════════════════╣
║  /update           — Download and install latest update  ║
║  /rollback         — Restore last stable version         ║
║  /status           — Show system status                  ║
║  /help             — Show this help                      ║
║  /quit             — Exit                                ║
╠══════════════════════════════════════════════════════════╣
║  Any other input → sent to AI as task                    ║
╚══════════════════════════════════════════════════════════╝
"""

async def interactive_loop(orch: AgentOS):
    print(HELP_TEXT)
    loop = asyncio.get_event_loop()
    while True:
        try:
            cmd = await loop.run_in_executor(None, lambda: input("SPS▶ ").strip())
        except (EOFError, KeyboardInterrupt):
            break
        if not cmd: continue

        if cmd in ("/quit","/exit","q"):
            break
        elif cmd == "/help":
            print(HELP_TEXT)
        elif cmd == "/status":
            print(json.dumps(orch.status(), indent=2, ensure_ascii=False))
        elif cmd == "/update":
            print("⏳ Updating...")
            print(await orch.self_update())
        elif cmd == "/rollback":
            print(await orch.rollback())
        else:
            res = await orch.execute(cmd)
            if res.get("success"):
                out = str(res.get("output","") or res.get("results","") or
                          res.get("description","") or "Done")
                print(f"✅ {out[:600]}")
            else:
                print(f"❌ {res.get('error','Unknown error')}")


# ══════════════════════════════════════════════════════════════════
#  [NEW] COLAB NOTEBOOK HELPER
# ══════════════════════════════════════════════════════════════════
def setup_notebook(groq_key: str = "", telegram_token: str = ""):
    """
    Call this from a Google Colab cell to initialize SPS:

        from sps_v60_2 import setup_notebook
        setup_notebook(groq_key="gsk_...", telegram_token="...")

    Then run:
        import asyncio
        orch = await start_sps()
    """
    AutoInstaller.setup_colab()
    if groq_key:
        os.environ["GROQ_API_KEY"] = groq_key
        CFG["groq_api_key"]        = groq_key
    if telegram_token:
        os.environ["TELEGRAM_TOKEN"] = telegram_token
        CFG["telegram_token"]        = telegram_token
    print("✅ SPS v60.2 Colab setup complete!")
    print("📌 Next step: orch = await start_sps()")


async def start_sps(skip_heavy: bool = IS_LOW_MEMORY) -> "AgentOS":
    """Async factory — use in Colab: orch = await start_sps()"""
    orch = AgentOS()
    await orch.initialize(skip_heavy=skip_heavy)
    return orch


# ══════════════════════════════════════════════════════════════════
#  MAIN
# ══════════════════════════════════════════════════════════════════
async def main_async(args):
    # [NEW] --setup flag: install all dependencies and exit
    if getattr(args, "setup", False):
        AutoInstaller.setup_all()
        return

    # [NEW] Colab mode: apply nest_asyncio
    if getattr(args, "colab", False) or IS_COLAB:
        AutoInstaller.setup_colab()

    skip_heavy = getattr(args, "low_memory", False) or IS_LOW_MEMORY

    orch = AgentOS()
    await orch.initialize(skip_heavy=skip_heavy)

    if getattr(args, "task", None):
        res = await orch.execute(args.task)
        print(json.dumps(res, indent=2, ensure_ascii=False))
    elif getattr(args, "daemon", False):
        try: await asyncio.Event().wait()
        except asyncio.CancelledError: pass
    else:
        await interactive_loop(orch)

    await orch.shutdown()


def main():
    p = argparse.ArgumentParser(description="SPS v60.2 — Autonomous Agent OS")
    p.add_argument("task",          nargs="?",          help="Single task to execute then exit")
    p.add_argument("--daemon",      action="store_true", help="Run as background daemon")
    p.add_argument("--status",      action="store_true", help="Print dependency status and exit")
    p.add_argument("--setup",       action="store_true", help="Install all optional dependencies and exit")
    p.add_argument("--colab",       action="store_true", help="Force Google Colab mode")
    p.add_argument("--low-memory",  action="store_true", dest="low_memory",
                   help="Force low-memory mode (skip heavy agents)")
    p.add_argument("--no-agents",   action="store_true", dest="no_agents",
                   help="Skip all optional agents (minimal mode)")
    args = p.parse_args()

    if args.status:
        print(f"Platform   : {PLATFORM_TAG}")
        print(f"Low-memory : {IS_LOW_MEMORY}")
        print(f"Colab      : {IS_COLAB}")
        print(f"aiohttp    : {'✅' if HAS_AIOHTTP   else '❌'}")
        print(f"numpy      : {'✅' if HAS_NUMPY      else '❌'}")
        print(f"FAISS      : {'✅' if HAS_FAISS      else '❌'}")
        print(f"Playwright : {'✅' if HAS_PLAYWRIGHT else '❌'}")
        print(f"BS4        : {'✅' if HAS_BS4        else '❌'}")
        print(f"Tesseract  : {'✅' if HAS_TESSERACT  else '❌'}")
        print(f"Whisper    : {'✅' if HAS_WHISPER    else '❌'}")
        print(f"edge-tts   : {'✅' if HAS_EDGE_TTS   else '❌'}")
        print(f"pyttsx3    : {'✅' if HAS_PYTTSX3    else '❌'}")
        print(f"Telegram   : {'✅' if HAS_TELEGRAM   else '❌'}")
        print(f"IPyAudio   : {'✅' if HAS_IPYTHON_AUDIO else '❌'}")
        print(f"Groq key   : {'✅' if CFG['groq_api_key']   else '❌'}")
        print(f"Gmail      : {'✅' if CFG['gmail_email']     else '❌'}")
        return

    if args.no_agents:
        # Override LOW_MEMORY_SKIP_AGENTS to skip everything heavy
        LOW_MEMORY_SKIP_AGENTS.update({"voice","ocr","vision","browser","mobile","gmail","calendar"})

    # [NEW] Handle Colab event loop conflict
    if IS_COLAB or args.colab:
        try:
            import nest_asyncio
            nest_asyncio.apply()
        except ImportError:
            subprocess.run([sys.executable,"-m","pip","install","nest_asyncio","-q"])
            try:
                import nest_asyncio
                nest_asyncio.apply()
            except Exception: pass
        try:
            loop = asyncio.get_running_loop()
            loop.run_until_complete(main_async(args))
        except RuntimeError:
            asyncio.run(main_async(args))
    else:
        asyncio.run(main_async(args))


if __name__ == "__main__":
    main()

