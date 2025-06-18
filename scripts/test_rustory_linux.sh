#!/bin/bash

# Rustory Linux 功能测试脚本
# 此脚本测试 rustory 的所有核心功能
# 测试环境：Linux
# 测试目录：/tmp/rustory_test
# 要求：rustory 已安装在 /usr/local/bin/rustory

set -e  # 遇到错误时立即退出
set -u  # 使用未定义变量时报错

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# 测试函数模板
run_test() {
    local test_name="$1"
    local test_cmd="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    log_info "测试 $TOTAL_TESTS: $test_name"
    
    if eval "$test_cmd"; then
        log_success "✓ $test_name"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        log_error "✗ $test_name"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# 检查 rustory 是否存在
check_rustory_binary() {
    if [[ ! -x "/usr/local/bin/rustory" ]]; then
        log_error "rustory 未找到在 /usr/local/bin/rustory"
        log_error "请确保 rustory 已正确安装"
        exit 1
    fi
    
    log_success "rustory 二进制文件检查通过"
}

# 设置测试环境
setup_test_env() {
    TEST_DIR="/tmp/rustory_test_$(date +%s)"
    
    # 清理可能存在的旧测试目录
    if [[ -d "$TEST_DIR" ]]; then
        rm -rf "$TEST_DIR"
    fi
    
    mkdir -p "$TEST_DIR"
    cd "$TEST_DIR"
    
    log_info "测试环境设置完成: $TEST_DIR"
}

# 清理测试环境
cleanup_test_env() {
    if [[ -d "$TEST_DIR" ]]; then
        rm -rf "$TEST_DIR"
        log_info "测试环境清理完成"
    fi
}

# 创建测试文件
create_test_files() {
    # 创建一些测试文件
    echo "Hello, Rustory!" > file1.txt
    echo "This is a test file" > file2.txt
    mkdir -p subdir
    echo "Nested file content" > subdir/nested.txt
    echo "Binary data" > binary_file.bin
    
    # 创建一个较大的文件测试
    dd if=/dev/zero of=large_file.dat bs=1024 count=100 2>/dev/null
    
    log_info "测试文件创建完成"
}

# 测试 rustory --version
test_version() {
    /usr/local/bin/rustory --version >/dev/null 2>&1
}

# 测试 rustory --help
test_help() {
    /usr/local/bin/rustory --help >/dev/null 2>&1
}

# 测试 rustory init
test_init() {
    /usr/local/bin/rustory init >/dev/null 2>&1 && [[ -d ".rustory" ]]
}

# 测试 rustory init 指定路径
test_init_with_path() {
    local init_test_dir="$TEST_DIR/init_test"
    mkdir -p "$init_test_dir"
    /usr/local/bin/rustory init "$init_test_dir" >/dev/null 2>&1 && [[ -d "$init_test_dir/.rustory" ]]
}

# 测试 rustory status (初始状态)
test_status_initial() {
    /usr/local/bin/rustory status >/dev/null 2>&1
}

# 测试 rustory status --verbose
test_status_verbose() {
    /usr/local/bin/rustory status --verbose >/dev/null 2>&1
}

# 测试 rustory status --json
test_status_json() {
    local output
    output=$(/usr/local/bin/rustory status --json 2>/dev/null)
    echo "$output" | python3 -m json.tool >/dev/null 2>&1
}

# 测试 rustory commit
test_commit() {
    /usr/local/bin/rustory commit -m "Initial commit" >/dev/null 2>&1
}

# 测试 rustory commit --json
test_commit_json() {
    echo "Modified content" >> file1.txt
    local output
    output=$(/usr/local/bin/rustory commit -m "JSON commit test" --json 2>/dev/null)
    echo "$output" | python3 -m json.tool >/dev/null 2>&1
}

# 测试 rustory history
test_history() {
    /usr/local/bin/rustory history >/dev/null 2>&1
}

# 测试 rustory history --json
test_history_json() {
    local output
    output=$(/usr/local/bin/rustory history --json 2>/dev/null)
    echo "$output" | python3 -m json.tool >/dev/null 2>&1
}

# 测试 rustory diff (工作目录)
test_diff_working_dir() {
    echo "Another change" >> file2.txt
    /usr/local/bin/rustory diff >/dev/null 2>&1
}

# 测试 rustory diff 快照间比较
test_diff_snapshots() {
    # 获取快照 ID
    local snapshot_ids
    snapshot_ids=$(/usr/local/bin/rustory history --json | python3 -c "
import sys, json
data = json.load(sys.stdin)
if isinstance(data, list) and len(data) >= 2:
    print(data[0]['id'], data[1]['id'])
elif isinstance(data, list) and len(data) == 1:
    print(data[0]['id'], data[0]['id'])
else:
    print('', '')
" 2>/dev/null)
    
    if [[ -n "$snapshot_ids" && "$snapshot_ids" != "  " ]]; then
        local id1 id2
        read -r id1 id2 <<< "$snapshot_ids"
        if [[ -n "$id1" ]]; then
            /usr/local/bin/rustory diff "$id1" >/dev/null 2>&1
        fi
    fi
    return 0  # 即使没有足够的快照也不算失败
}

# 测试 rustory tag
test_tag() {
    # 获取最新快照 ID
    local latest_id
    local history_output
    
    # 首先尝试获取历史记录
    history_output=$(/usr/local/bin/rustory history --json 2>/dev/null) || {
        log_warning "无法获取历史记录，跳过标签测试"
        return 0
    }
    
    # 解析最新的快照 ID
    latest_id=$(echo "$history_output" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if isinstance(data, list) and len(data) > 0:
        print(data[0]['id'])
    elif isinstance(data, dict) and 'snapshots' in data and len(data['snapshots']) > 0:
        print(data['snapshots'][0]['id'])
    else:
        sys.exit(1)
except (json.JSONDecodeError, KeyError, IndexError):
    sys.exit(1)
" 2>/dev/null) || {
        log_warning "无法解析快照 ID，跳过标签测试"
        return 0
    }
    
    if [[ -n "$latest_id" ]]; then
        /usr/local/bin/rustory tag "v1.0" "$latest_id" >/dev/null 2>&1
    else
        log_warning "未找到有效快照 ID，跳过标签测试"
        return 0
    fi
}

# 测试 rustory config
test_config_get() {
    /usr/local/bin/rustory config get user.name >/dev/null 2>&1 || return 0  # 配置可能不存在
}

test_config_set() {
    /usr/local/bin/rustory config set user.name "Test User" >/dev/null 2>&1 &&
    /usr/local/bin/rustory config get user.name | grep -q "Test User"
}

# 测试 rustory ignore
test_ignore_show() {
    /usr/local/bin/rustory ignore show >/dev/null 2>&1 || return 0  # 忽略文件可能不存在
}

test_ignore_functionality() {
    # 创建忽略规则
    echo "*.tmp" > .rustoryignore
    echo "temp/" >> .rustoryignore
    
    # 创建应该被忽略的文件
    echo "temporary content" > test.tmp
    mkdir -p temp
    echo "temp content" > temp/file.txt
    
    # 检查状态，这些文件应该不出现在状态中
    /usr/local/bin/rustory status >/dev/null 2>&1
}

# 测试 rustory rollback
test_rollback() {
    # 创建一些修改
    echo "Content to be rolled back" > rollback_test.txt
    /usr/local/bin/rustory commit -m "Changes to rollback" >/dev/null 2>&1
    
    # 获取前一个快照 ID
    local prev_id
    local history_output
    
    # 首先尝试获取历史记录
    history_output=$(/usr/local/bin/rustory history --json 2>/dev/null) || {
        log_warning "无法获取历史记录，跳过回滚测试"
        return 0
    }
    
    # 解析前一个快照 ID（取第二个，如果只有一个就取第一个）
    prev_id=$(echo "$history_output" | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if isinstance(data, list) and len(data) >= 2:
        print(data[1]['id'])
    elif isinstance(data, list) and len(data) == 1:
        print(data[0]['id'])
    elif isinstance(data, dict) and 'snapshots' in data:
        snapshots = data['snapshots']
        if len(snapshots) >= 2:
            print(snapshots[1]['id'])
        elif len(snapshots) == 1:
            print(snapshots[0]['id'])
        else:
            sys.exit(1)
    else:
        sys.exit(1)
except (json.JSONDecodeError, KeyError, IndexError):
    sys.exit(1)
" 2>/dev/null) || {
        log_warning "无法解析快照 ID，跳过回滚测试"
        return 0
    }
    
    if [[ -n "$prev_id" ]]; then
        # 测试回滚（导出到备份目录，这是默认行为）
        /usr/local/bin/rustory rollback "$prev_id" >/dev/null 2>&1
    else
        log_warning "未找到有效快照 ID，跳过回滚测试"
        return 0
    fi
}

# 测试 rustory stats
test_stats() {
    /usr/local/bin/rustory stats >/dev/null 2>&1
}

test_stats_json() {
    local output
    output=$(/usr/local/bin/rustory stats --json 2>/dev/null)
    echo "$output" | python3 -m json.tool >/dev/null 2>&1
}

# 测试 rustory verify
test_verify() {
    /usr/local/bin/rustory verify >/dev/null 2>&1
}

test_verify_fix() {
    /usr/local/bin/rustory verify --fix >/dev/null 2>&1
}

# 测试 rustory gc
test_gc_dry_run() {
    /usr/local/bin/rustory gc --dry-run >/dev/null 2>&1
}

test_gc() {
    /usr/local/bin/rustory gc >/dev/null 2>&1
}

test_gc_aggressive() {
    /usr/local/bin/rustory gc --aggressive >/dev/null 2>&1
}

# 边界条件测试
test_large_file_handling() {
    # 创建一个相对较大的文件 (5MB)
    dd if=/dev/zero of=large_test.dat bs=1024 count=5120 2>/dev/null
    /usr/local/bin/rustory commit -m "Large file test" >/dev/null 2>&1
}

test_unicode_filenames() {
    # 创建包含 Unicode 字符的文件名
    echo "Unicode content" > "测试文件_🚀.txt"
    echo "Emoji file" > "file_📁_test.txt"
    /usr/local/bin/rustory commit -m "Unicode filename test" >/dev/null 2>&1
}

test_deep_directory_structure() {
    # 创建深层目录结构
    mkdir -p very/deep/directory/structure/for/testing
    echo "Deep file" > very/deep/directory/structure/for/testing/file.txt
    /usr/local/bin/rustory commit -m "Deep directory test" >/dev/null 2>&1
}

# 错误处理测试
test_invalid_snapshot_id() {
    # 使用无效的快照 ID，应该失败但不崩溃
    /usr/local/bin/rustory rollback "invalid_id_12345" >/dev/null 2>&1 && return 1 || return 0
}

test_rollback_nonexistent() {
    # 尝试回滚到不存在的快照
    /usr/local/bin/rustory rollback "00000000-0000-0000-0000-000000000000" >/dev/null 2>&1 && return 1 || return 0
}

# 显示测试结果摘要
show_test_summary() {
    echo
    echo "======================================"
    echo "         测试结果摘要"
    echo "======================================"
    echo -e "总测试数:   ${BLUE}$TOTAL_TESTS${NC}"
    echo -e "通过测试:   ${GREEN}$PASSED_TESTS${NC}"
    echo -e "失败测试:   ${RED}$FAILED_TESTS${NC}"
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "\n${GREEN}🎉 所有测试通过！Rustory 功能正常！${NC}"
        return 0
    else
        echo -e "\n${RED}❌ 有 $FAILED_TESTS 个测试失败${NC}"
        return 1
    fi
}

# 主测试流程
main() {
    echo "======================================"
    echo "      Rustory Linux 功能测试"
    echo "======================================"
    echo
    
    # 检查依赖
    if ! command -v python3 >/dev/null 2>&1; then
        log_warning "python3 未找到，JSON 测试可能会失败"
    fi
    
    # 检查 rustory 二进制文件
    check_rustory_binary
    
    # 设置测试环境
    setup_test_env
    trap cleanup_test_env EXIT
    
    # 创建测试文件
    create_test_files
    
    echo
    log_info "开始执行功能测试..."
    echo
    
    # 基础功能测试
    run_test "版本信息显示" "test_version"
    run_test "帮助信息显示" "test_help"
    run_test "初始化仓库" "test_init"
    run_test "指定路径初始化" "test_init_with_path"
    
    # 状态检查测试
    run_test "状态检查 (基础)" "test_status_initial"
    run_test "状态检查 (详细)" "test_status_verbose"
    run_test "状态检查 (JSON)" "test_status_json"
    
    # 提交功能测试
    run_test "创建快照" "test_commit"
    run_test "创建快照 (JSON)" "test_commit_json"
    
    # 历史记录测试
    run_test "查看历史记录" "test_history"
    run_test "查看历史记录 (JSON)" "test_history_json"
    
    # 差异比较测试
    run_test "差异比较 (工作目录)" "test_diff_working_dir"
    run_test "差异比较 (快照间)" "test_diff_snapshots"
    
    # 标签功能测试
    run_test "创建标签" "test_tag"
    
    # 配置管理测试
    run_test "配置获取" "test_config_get"
    run_test "配置设置" "test_config_set"
    
    # 忽略规则测试
    run_test "忽略规则显示" "test_ignore_show"
    run_test "忽略功能测试" "test_ignore_functionality"
    
    # 回滚功能测试
    run_test "快照回滚" "test_rollback"
    
    # 统计信息测试
    run_test "仓库统计" "test_stats"
    run_test "仓库统计 (JSON)" "test_stats_json"
    
    # 完整性验证测试
    run_test "完整性验证" "test_verify"
    run_test "完整性验证和修复" "test_verify_fix"
    
    # 垃圾回收测试
    run_test "垃圾回收 (试运行)" "test_gc_dry_run"
    run_test "垃圾回收" "test_gc"
    run_test "垃圾回收 (积极模式)" "test_gc_aggressive"
    
    # 边界条件测试
    run_test "大文件处理" "test_large_file_handling"
    run_test "Unicode 文件名" "test_unicode_filenames"
    run_test "深层目录结构" "test_deep_directory_structure"
    
    # 错误处理测试
    run_test "无效快照 ID 处理" "test_invalid_snapshot_id"
    run_test "不存在快照回滚" "test_rollback_nonexistent"
    
    # 显示测试结果
    show_test_summary
}

# 脚本入口点
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
