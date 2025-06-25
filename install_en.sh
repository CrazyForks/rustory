#!/bin/bash

# Rustory one-click installation script
# For Linux/macOS systems

set -e

# Script version
SCRIPT_VERSION="1.0.0"

# Project info
PROJECT_NAME="rustory"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="rustory"
GITHUB_REPO="uselibrary/rustory"
GITHUB_API_URL="https://api.github.com/repos/${GITHUB_REPO}/releases/latest"

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Log functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_debug() {
    echo -e "${BLUE}[DEBUG]${NC} $1"
}

# Show help information
show_help() {
    cat << EOF
Rustory Installation Script v${SCRIPT_VERSION}

Usage: $0 [option]

Options:
    install     Install or update rustory
    uninstall   Uninstall rustory
    --help      Show this help message
    --version   Show script version

Examples:
    $0 install      # Install rustory
    $0 uninstall    # Uninstall rustory

EOF
}

# Check operating system
check_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        log_info "Detected Linux system"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        log_info "Detected macOS system"
    else
        log_error "Unsupported operating system: $OSTYPE"
        exit 1
    fi
}

# Check system architecture
check_arch() {
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            log_error "Unsupported system architecture: $ARCH"
            exit 1
            ;;
    esac
    log_info "Detected system architecture: $ARCH"
}

# Check for root privileges
check_root() {
    if [ "$EUID" -ne 0 ]; then
        log_error "Root privileges are required to install to ${INSTALL_DIR}"
        log_info "Please run this script with sudo: sudo $0 $1"
        exit 1
    fi
}

# Check required dependencies
check_dependencies() {
    local deps=("curl" "tar" "gzip" "jq")
    local missing_deps=()
    
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        log_info "Please install these dependencies first:"
        
        if [[ "$OS" == "linux" ]]; then
            if command -v apt-get &> /dev/null; then
                log_info "  sudo apt-get update && sudo apt-get install -y ${missing_deps[*]}"
            elif command -v yum &> /dev/null; then
                log_info "  sudo yum install -y ${missing_deps[*]}"
            elif command -v dnf &> /dev/null; then
                log_info "  sudo dnf install -y ${missing_deps[*]}"
            elif command -v pacman &> /dev/null; then
                log_info "  sudo pacman -S ${missing_deps[*]}"
            fi
        elif [[ "$OS" == "macos" ]]; then
            if command -v brew &> /dev/null; then
                log_info "  brew install ${missing_deps[*]}"
            else
                log_info "  Please install Homebrew or install these dependencies manually"
            fi
        fi
        exit 1
    fi
}

# Get currently installed version
get_installed_version() {
    if command -v "$BINARY_NAME" &> /dev/null; then
        local version_output
        version_output=$("$BINARY_NAME" --version 2>/dev/null || echo "")
        if [[ "$version_output" =~ rustory[[:space:]]+([0-9]+\.[0-9]+\.[0-9]+) ]]; then
            echo "${BASH_REMATCH[1]}"
        else
            echo ""
        fi
    else
        echo ""
    fi
}

# Get latest version from GitHub API
get_latest_version() {
    local response
    response=$(curl -s --connect-timeout 10 --max-time 30 "$GITHUB_API_URL" 2>/dev/null)
    
    if [[ $? -ne 0 || -z "$response" ]]; then
        return 1
    fi
    
    # Check for jq
    if command -v jq &> /dev/null; then
        local version
        version=$(echo "$response" | jq -r '.tag_name // empty' 2>/dev/null)
        if [[ -n "$version" && "$version" != "null" ]]; then
            # Remove possible 'v' prefix
            version=${version#v}
            echo "$version"
            return 0
        fi
    fi
    
    # If no jq or parse failed, use regex
    local version
    version=$(echo "$response" | grep -o '"tag_name":"[^"]*"' | head -1 | sed 's/"tag_name":"//;s/"//')
    if [[ -n "$version" ]]; then
        # Remove possible 'v' prefix
        version=${version#v}
        echo "$version"
        return 0
    fi
    
    return 1
}

# Get download URL for specified version
get_download_url() {
    local version="$1"
    local archive_name="$2"
    
    # Build download URL
    echo "https://github.com/${GITHUB_REPO}/releases/download/v${version}/${archive_name}"
}

# Version comparison function
version_compare() {
    local version1=$1
    local version2=$2
    
    if [[ "$version1" == "$version2" ]]; then
        return 0  # equal
    fi
    
    local IFS=.
    local i ver1=($version1) ver2=($version2)
    
    # Fill version arrays
    for ((i=${#ver1[@]}; i<${#ver2[@]}; i++)); do
        ver1[i]=0
    done
    for ((i=${#ver2[@]}; i<${#ver1[@]}; i++)); do
        ver2[i]=0
    done
    
    # Compare versions
    for ((i=0; i<${#ver1[@]}; i++)); do
        if [[ -z ${ver2[i]} ]]; then
            ver2[i]=0
        fi
        if ((10#${ver1[i]} > 10#${ver2[i]})); then
            return 1  # version1 > version2
        fi
        if ((10#${ver1[i]} < 10#${ver2[i]})); then
            return 2  # version1 < version2
        fi
    done
    return 0  # equal
}

# Download and install rustory
download_and_install() {
    local latest_version
    latest_version=$(get_latest_version)
    
    if [[ $? -ne 0 || -z "$latest_version" ]]; then
        log_error "Unable to retrieve the latest version information"
        exit 1
    fi
    
    log_info "Latest version: $latest_version"
    
    local archive_name=""
    
    # Determine download file name based on OS and architecture
    if [[ "$OS" == "linux" ]]; then
        if [[ "$ARCH" == "x86_64" ]]; then
            archive_name="rustory-x86_64-unknown-linux-musl.tar.gz"
        elif [[ "$ARCH" == "aarch64" ]]; then
            archive_name="rustory-aarch64-unknown-linux-musl.tar.gz"
        fi
    elif [[ "$OS" == "macos" ]]; then
        if [[ "$ARCH" == "x86_64" ]]; then
            archive_name="rustory-x86_64-apple-darwin.tar.gz"
        elif [[ "$ARCH" == "aarch64" ]]; then
            archive_name="rustory-aarch64-apple-darwin.tar.gz"
        fi
    fi
    
    if [[ -z "$archive_name" ]]; then
        log_error "Unsupported system configuration: $OS-$ARCH"
        exit 1
    fi
    
    local download_url
    download_url=$(get_download_url "$latest_version" "$archive_name")
    
    log_info "Downloading $archive_name..."
    log_debug "Download URL: $download_url"
    
    local temp_dir=$(mktemp -d)
    local archive_path="$temp_dir/$archive_name"
    
    # Download file
    if ! curl -L -o "$archive_path" "$download_url" --connect-timeout 10 --max-time 300; then
        log_error "Download failed"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Verify downloaded file
    if [[ ! -f "$archive_path" ]]; then
        log_error "Downloaded file not found"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Check file size
    local file_size
    file_size=$(stat -c%s "$archive_path" 2>/dev/null || stat -f%z "$archive_path" 2>/dev/null || echo "0")
    if [[ "$file_size" -lt 1000 ]]; then
        log_error "The downloaded file seems incomplete"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Extract file
    log_info "Extracting..."
    if ! tar -xzf "$archive_path" -C "$temp_dir"; then
        log_error "Extraction failed"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Find binary file
    local binary_path
    binary_path=$(find "$temp_dir" -name "$BINARY_NAME" -type f | head -1)
    
    if [[ -z "$binary_path" || ! -f "$binary_path" ]]; then
        log_error "Executable file not found in the extracted files"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Install binary file
    log_info "Installing $BINARY_NAME to $INSTALL_DIR..."
    cp "$binary_path" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    # Create symlink rty -> rustory
    log_info "Creating symlink rty -> rustory..."
    local symlink_path="$INSTALL_DIR/rty"
    
    # Remove existing symlink if present
    if [[ -L "$symlink_path" || -f "$symlink_path" ]]; then
        rm -f "$symlink_path"
    fi
    
    # Create new symlink
    if ln -s "$INSTALL_DIR/$BINARY_NAME" "$symlink_path"; then
        log_info "Symlink created: $symlink_path -> $INSTALL_DIR/$BINARY_NAME"
    else
        log_warn "Symlink creation failed, but main program installed successfully"
    fi
    
    # Clean up temp files
    rm -rf "$temp_dir"
    
    log_info "Installation complete!"
    return 0
}

# Install function
install_rustory() {
    log_info "Starting rustory installation..."
    
    # Get latest version
    log_info "Fetching latest version info..."
    local latest_version
    latest_version=$(get_latest_version)
    
    if [[ $? -ne 0 || -z "$latest_version" ]]; then
        log_error "Failed to get latest version info"
        exit 1
    fi
    
    log_info "Latest available version: $latest_version"
    
    # Check if already installed
    local installed_version
    installed_version=$(get_installed_version)
    
    if [[ -n "$installed_version" ]]; then
        log_info "Detected installed version: $installed_version, will force install latest version ($latest_version)..."
    else
        log_info "Rustory not detected, installing latest version ($latest_version)..."
    fi
    
    download_and_install
    
    # Verify installation
    if command -v "$BINARY_NAME" &> /dev/null; then
        local new_version
        new_version=$("$BINARY_NAME" --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' || echo "unknown")
        log_info "Installation successful! Version: $new_version"
        log_info "Use '$BINARY_NAME --help' for help"
        
        # Verify symlink
        if command -v "rty" &> /dev/null; then
            log_info "Symlink 'rty' is also available"
        else
            log_warn "Symlink 'rty' verification failed, but main program works fine"
        fi
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# Uninstall function
uninstall_rustory() {
    log_info "Starting rustory uninstallation..."
    
    if [[ ! -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        log_warn "rustory is not installed or not in the expected location"
        
        # Check if only symlink exists
        local symlink_path="$INSTALL_DIR/rty"
        if [[ -L "$symlink_path" ]]; then
            log_info "Found orphaned symlink, cleaning up"
            rm -f "$symlink_path"
        fi
        
        return 0
    fi
    
    # Confirm uninstallation
    read -p "Are you sure you want to uninstall rustory? [y/N]: " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Uninstallation cancelled"
        return 0
    fi
    
    # Remove binary file
    rm -f "$INSTALL_DIR/$BINARY_NAME"
    
    # Remove symlink
    local symlink_path="$INSTALL_DIR/rty"
    if [[ -L "$symlink_path" ]]; then
        log_info "Removing symlink: $symlink_path"
        rm -f "$symlink_path"
    fi
    
    if [[ ! -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
        log_info "Uninstallation complete!"
    else
        log_error "Uninstallation failed"
        exit 1
    fi
}

# Main function
main() {
    case "${1:-install}" in
        "install")
            check_os
            check_arch
            check_root "install"
            check_dependencies
            install_rustory
            ;;
        "uninstall")
            check_root "uninstall"
            uninstall_rustory
            ;;
        "--help"|"-h")
            show_help
            ;;
        "--version"|"-v")
            echo "Rustory Installation Script v${SCRIPT_VERSION}"
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"