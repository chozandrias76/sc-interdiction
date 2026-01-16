#!/usr/bin/env python3
"""
Star Citizen game data extraction script.

Provides two extraction methods:
1. Primary: Clone pre-extracted JSON from scunpacked-data repository
   (Works reliably, community-maintained, updated with game patches)

2. Fallback: scdatatools for direct p4k extraction
   (May not work with latest game versions due to format changes)

Output files:
- items.json - All game items with properties
- labels.json - Localization strings (English)
- ships.json - Ship definitions
- fps-items.json - FPS/player equipment
- ship-items.json - Ship components

Prerequisites:
    git (for scunpacked-data clone)
    OR
    pip install scdatatools (for direct extraction)

Usage:
    python scripts/extract_gamedata.py --help
    python scripts/extract_gamedata.py  # Uses scunpacked-data (recommended)
    python scripts/extract_gamedata.py --from-p4k  # Try direct extraction
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path


DEFAULT_P4K_PATH = "/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k"
DEFAULT_OUTPUT_DIR = "./extracted"
SCUNPACKED_REPO = "https://github.com/StarCitizenWiki/scunpacked-data.git"


def log(msg: str) -> None:
    """Print message to stderr for progress reporting."""
    print(msg, file=sys.stderr)


def check_git() -> bool:
    """Verify git is installed."""
    try:
        result = subprocess.run(
            ["git", "--version"],
            capture_output=True,
            text=True,
            check=False,
        )
        return result.returncode == 0
    except FileNotFoundError:
        return False


def check_scdatatools() -> bool:
    """Verify scdatatools is installed and accessible."""
    try:
        result = subprocess.run(
            ["scdt", "--help"],
            capture_output=True,
            text=True,
            check=False,
        )
        return result.returncode == 0
    except FileNotFoundError:
        return False


def clone_scunpacked_data(output_dir: Path) -> bool:
    """Clone or update scunpacked-data repository.

    This is the recommended extraction method as it contains
    pre-extracted and validated JSON data maintained by the
    Star Citizen Wiki community.
    """
    scunpacked_dir = output_dir / "scunpacked-data"

    if scunpacked_dir.exists():
        # Update existing clone
        log(f"Updating existing scunpacked-data at {scunpacked_dir}...")
        cmd = ["git", "-C", str(scunpacked_dir), "pull", "--ff-only"]
    else:
        # Fresh clone
        log(f"Cloning scunpacked-data to {scunpacked_dir}...")
        log("This contains pre-extracted JSON from Star Citizen game files.")
        output_dir.mkdir(parents=True, exist_ok=True)
        cmd = ["git", "clone", "--depth", "1", SCUNPACKED_REPO, str(scunpacked_dir)]

    log(f"Running: {' '.join(cmd)}")

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=False,
        )
        if result.returncode != 0:
            log(f"Git command failed: {result.stderr}")
            return False
        return True
    except Exception as e:
        log(f"Error with git: {e}")
        return False


def extract_dataforge(p4k_path: str, output_dir: Path) -> bool:
    """Extract DataForge records as JSON files using scdatatools.

    Note: This may fail with newer game versions due to p4k format changes.
    The scunpacked-data approach is more reliable.
    """
    forge_output = output_dir / "dataforge"
    forge_output.mkdir(parents=True, exist_ok=True)

    log(f"Extracting DataForge records to {forge_output}...")
    log("This may take several minutes for large archives.")

    cmd = [
        "scdt", "forge", "extract",
        p4k_path,
        "--json",
        "--output", str(forge_output),
        "-v",
    ]

    log(f"Running: {' '.join(cmd)}")

    try:
        result = subprocess.run(
            cmd,
            capture_output=False,
            check=False,
        )
        if result.returncode != 0:
            log(f"Warning: DataForge extraction returned code {result.returncode}")
            return False
        return True
    except Exception as e:
        log(f"Error extracting DataForge: {e}")
        return False


def extract_localization(p4k_path: str, output_dir: Path) -> bool:
    """Extract localization strings using scdatatools."""
    output_file = output_dir / "localization.csv"
    output_dir.mkdir(parents=True, exist_ok=True)

    log(f"Extracting localization to {output_file}...")

    cmd = [
        "scdt", "localization", "export",
        p4k_path,
        str(output_file),
        "--languages", "english",
        "-v",
    ]

    log(f"Running: {' '.join(cmd)}")

    try:
        result = subprocess.run(
            cmd,
            capture_output=False,
            check=False,
        )
        if result.returncode != 0:
            log(f"Warning: Localization extraction returned code {result.returncode}")
            return False
        return True
    except Exception as e:
        log(f"Error extracting localization: {e}")
        return False


def verify_scunpacked_data(output_dir: Path) -> dict:
    """Verify scunpacked-data extraction and return file info."""
    scunpacked_dir = output_dir / "scunpacked-data"
    expected_files = [
        "items.json",
        "labels.json",
        "ships.json",
        "fps-items.json",
        "ship-items.json",
        "manufacturers.json",
    ]

    results = {}
    for filename in expected_files:
        filepath = scunpacked_dir / filename
        if filepath.exists():
            size = filepath.stat().st_size
            results[filename] = {
                "exists": True,
                "size": size,
                "size_human": f"{size / (1024*1024):.1f}MB" if size > 1024*1024 else f"{size / 1024:.1f}KB",
            }
        else:
            results[filename] = {"exists": False}

    return results


def main():
    parser = argparse.ArgumentParser(
        description="Extract Star Citizen game data for sc-interdiction project",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Extraction Methods:
    Default: Clone scunpacked-data repository (recommended)
             Contains pre-extracted JSON maintained by Star Citizen Wiki

    --from-p4k: Direct extraction using scdatatools
                May fail with newer game versions

Examples:
    # Clone/update scunpacked-data (recommended)
    python scripts/extract_gamedata.py

    # Try direct p4k extraction (requires scdatatools + compatible game version)
    python scripts/extract_gamedata.py --from-p4k --p4k "/path/to/Data.p4k"

Output:
    extracted/scunpacked-data/items.json     - All items (~46MB)
    extracted/scunpacked-data/labels.json    - Localization (~10MB)
    extracted/scunpacked-data/ships.json     - Ships (~1.4MB)
    extracted/scunpacked-data/fps-items.json - FPS equipment (~19MB)
""",
    )

    parser.add_argument(
        "--from-p4k",
        action="store_true",
        help="Extract directly from Data.p4k using scdatatools (may fail with newer versions)",
    )

    parser.add_argument(
        "--p4k",
        default=DEFAULT_P4K_PATH,
        help=f"Path to Data.p4k file for --from-p4k mode (default: {DEFAULT_P4K_PATH})",
    )

    parser.add_argument(
        "--output", "-o",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Output directory (default: {DEFAULT_OUTPUT_DIR})",
    )

    parser.add_argument(
        "--update",
        action="store_true",
        help="Force update of scunpacked-data even if it exists",
    )

    args = parser.parse_args()
    output_dir = Path(args.output)

    if args.from_p4k:
        # Direct p4k extraction mode
        log("=== Direct P4K Extraction Mode ===")
        log("Note: This may fail with newer game versions. Consider using scunpacked-data instead.")

        p4k_path = Path(args.p4k)
        if not p4k_path.exists():
            log(f"Error: Data.p4k not found at {p4k_path}")
            log("Please specify the correct path with --p4k")
            sys.exit(1)

        if not check_scdatatools():
            log("Error: scdatatools not found.")
            log("Install with: pip install scdatatools")
            log("Or use the default scunpacked-data method instead.")
            sys.exit(1)

        log(f"Using Data.p4k: {p4k_path}")
        log(f"Output directory: {args.output}")

        output_dir.mkdir(parents=True, exist_ok=True)

        df_success = extract_dataforge(str(p4k_path), output_dir)
        loc_success = extract_localization(str(p4k_path), output_dir)

        if df_success and loc_success:
            log("\nExtraction completed successfully!")
            sys.exit(0)
        else:
            log("\nExtraction failed. Consider using scunpacked-data instead:")
            log("  python scripts/extract_gamedata.py")
            sys.exit(1)

    else:
        # scunpacked-data mode (default)
        log("=== scunpacked-data Extraction Mode ===")
        log("Using pre-extracted JSON from Star Citizen Wiki community")

        if not check_git():
            log("Error: git not found. Please install git.")
            sys.exit(1)

        scunpacked_dir = output_dir / "scunpacked-data"

        if scunpacked_dir.exists() and not args.update:
            log(f"scunpacked-data already exists at {scunpacked_dir}")
            log("Use --update to pull latest changes")
        else:
            if not clone_scunpacked_data(output_dir):
                log("\nFailed to clone/update scunpacked-data")
                sys.exit(1)

        # Verify extraction
        log("\n=== Extraction Summary ===")
        results = verify_scunpacked_data(output_dir)

        all_exist = True
        for filename, info in results.items():
            if info["exists"]:
                log(f"  {filename}: {info['size_human']}")
            else:
                log(f"  {filename}: MISSING")
                all_exist = False

        if all_exist:
            log("\nExtraction completed successfully!")
            log(f"\nData available at: {scunpacked_dir}")
            sys.exit(0)
        else:
            log("\nSome files are missing. Try re-running with --update")
            sys.exit(1)


if __name__ == "__main__":
    main()
