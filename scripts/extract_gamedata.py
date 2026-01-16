#!/usr/bin/env python3
"""
Star Citizen game data extraction script.

Uses scdatatools to extract:
- DataForge records (Game2.dcb) as JSON files
- Localization strings (global.ini) as CSV

Prerequisites:
    pip install scdatatools

Usage:
    python scripts/extract_gamedata.py --help
    python scripts/extract_gamedata.py --p4k "/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k"
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path


DEFAULT_P4K_PATH = "/mnt/c/Star Citizen/StarCitizen/LIVE/Data.p4k"
DEFAULT_OUTPUT_DIR = "./extracted"


def log(msg: str) -> None:
    """Print message to stderr for progress reporting."""
    print(msg, file=sys.stderr)


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


def extract_dataforge(p4k_path: str, output_dir: Path) -> bool:
    """Extract DataForge records as JSON files.

    Uses scdt forge extract with JSON output format.
    Extracts all records from the DataForge database.
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
            capture_output=False,  # Stream output to terminal
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
    """Extract localization strings to CSV.

    Uses scdt localization export to get all translation keys.
    """
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


def extract_global_ini(p4k_path: str, output_dir: Path) -> bool:
    """Extract global.ini directly from p4k archive.

    Falls back to direct file extraction if localization export fails.
    """
    output_dir.mkdir(parents=True, exist_ok=True)

    log(f"Extracting global.ini to {output_dir}...")

    cmd = [
        "scdt", "p4k", "extract",
        p4k_path,
        "--output", str(output_dir),
        "--file-filter", "*/english/global.ini",
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
            log(f"Warning: global.ini extraction returned code {result.returncode}")
            return False
        return True
    except Exception as e:
        log(f"Error extracting global.ini: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Extract Star Citizen game data using scdatatools",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    # Extract with default paths
    python scripts/extract_gamedata.py

    # Specify custom p4k location
    python scripts/extract_gamedata.py --p4k "/path/to/Data.p4k"

    # Specify output directory
    python scripts/extract_gamedata.py --output ./my_extracted_data

    # Extract only specific data
    python scripts/extract_gamedata.py --dataforge-only
    python scripts/extract_gamedata.py --localization-only
""",
    )

    parser.add_argument(
        "--p4k",
        default=DEFAULT_P4K_PATH,
        help=f"Path to Data.p4k file (default: {DEFAULT_P4K_PATH})",
    )

    parser.add_argument(
        "--output", "-o",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Output directory for extracted files (default: {DEFAULT_OUTPUT_DIR})",
    )

    parser.add_argument(
        "--dataforge-only",
        action="store_true",
        help="Extract only DataForge records",
    )

    parser.add_argument(
        "--localization-only",
        action="store_true",
        help="Extract only localization data",
    )

    args = parser.parse_args()

    # Validate p4k path
    p4k_path = Path(args.p4k)
    if not p4k_path.exists():
        log(f"Error: Data.p4k not found at {p4k_path}")
        log("Please specify the correct path with --p4k")
        sys.exit(1)

    # Check scdatatools
    if not check_scdatatools():
        log("Error: scdatatools not found. Install with: pip install scdatatools")
        sys.exit(1)

    log(f"Using Data.p4k: {p4k_path}")
    log(f"Output directory: {args.output}")

    output_dir = Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)

    success = True

    # Determine what to extract
    extract_df = not args.localization_only
    extract_loc = not args.dataforge_only

    if extract_df:
        if not extract_dataforge(str(p4k_path), output_dir):
            success = False

    if extract_loc:
        # Try localization export first, fall back to direct extraction
        if not extract_localization(str(p4k_path), output_dir):
            log("Trying direct global.ini extraction...")
            if not extract_global_ini(str(p4k_path), output_dir):
                success = False

    # Summary
    log("\n=== Extraction Summary ===")

    if extract_df:
        forge_dir = output_dir / "dataforge"
        if forge_dir.exists():
            json_files = list(forge_dir.rglob("*.json"))
            log(f"DataForge: {len(json_files)} JSON files extracted")
        else:
            log("DataForge: FAILED")

    if extract_loc:
        loc_file = output_dir / "localization.csv"
        if loc_file.exists():
            lines = sum(1 for _ in open(loc_file))
            log(f"Localization: {lines} entries in {loc_file}")
        else:
            # Check for global.ini
            ini_files = list(output_dir.rglob("global.ini"))
            if ini_files:
                for ini in ini_files:
                    lines = sum(1 for _ in open(ini))
                    log(f"global.ini: {lines} lines in {ini}")
            else:
                log("Localization: FAILED")

    if success:
        log("\nExtraction completed successfully!")
        sys.exit(0)
    else:
        log("\nExtraction completed with warnings. Check output above.")
        sys.exit(1)


if __name__ == "__main__":
    main()
