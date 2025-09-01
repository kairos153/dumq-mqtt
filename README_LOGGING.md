# DumQ MQTT File Logging for Debugging

이 문서는 DumQ MQTT 라이브러리에서 디버깅을 위해 로그를 파일로 저장하는 방법을 설명합니다.

## 빠른 시작

### 기본 파일 로깅 설정

가장 간단한 방법은 `init_simple_file_logging` 함수를 사용하는 것입니다:

```rust
use dumq_mqtt::logging::init_simple_file_logging;

fn main() {
    // 콘솔과 파일 모두에 로그를 작성하도록 초기화
    let _handle = init_simple_file_logging(
        "debug.log",
        log::LevelFilter::Debug,
    ).unwrap();
    
    // MQTT 코드...
    log::info!("애플리케이션이 시작되었습니다");
    log::debug!("디버그 정보");
}
```

### 디렉토리 기반 로깅

더 체계적인 로깅을 위해 `init_file_logging` 함수를 사용할 수 있습니다:

```rust
use dumq_mqtt::logging::init_file_logging;

fn main() {
    // 'logs' 디렉토리를 생성하고 'logs/dumq_mqtt.log'에 작성
    let _handle = init_file_logging(
        "logs",
        "dumq_mqtt.log",
        log::LevelFilter::Info,
    ).unwrap();
    
    // MQTT 코드...
}
```

## 로그 레벨

로그 시스템은 다음 로그 레벨을 지원합니다 (가장 상세한 것부터):

- **trace**: 매우 상세한 디버깅 정보
- **debug**: 일반적인 디버깅 정보
- **info**: 프로그램 실행에 대한 일반적인 정보
- **warn**: 경고 메시지
- **error**: 오류 메시지

## 환경 변수 사용

`RUST_LOG` 환경 변수를 사용하여 로깅 동작을 제어할 수 있습니다:

```bash
# 모든 디버그 로그 활성화
export RUST_LOG=debug

# dumq_mqtt 디버그 로그만 활성화
export RUST_LOG=dumq_mqtt=debug

# 특정 모듈의 디버그 로그 활성화
export RUST_LOG=dumq_mqtt::server=debug,dumq_mqtt::client=info

# info 이상만 활성화
export RUST_LOG=info

# 경고와 오류만 활성화
export RUST_LOG=warn

# 오류만 활성화
export RUST_LOG=error
```

## 예제 실행

파일 로깅이 작동하는 것을 확인하려면 제공된 예제를 실행하세요:

```bash
# 파일 로깅 예제 실행
cargo run --example file_logging_example

# 생성된 로그 파일 확인
ls -la *.log logs/
```

## 로그 파일 형식

로그 항목은 다음 패턴으로 포맷됩니다:

```
{timestamp} [{level}] {target} - {message}
```

예시:
```
2024-01-15 14:30:45.123 [INFO] dumq_mqtt::server - Starting MQTT server on 127.0.0.1:1883
2024-01-15 14:30:45.124 [DEBUG] dumq_mqtt::codec - Decoding packet, buffer size: 48
2024-01-15 14:30:45.125 [ERROR] dumq_mqtt::client - Connection failed: Connection refused
```

## 기존 코드에 통합

이미 MQTT 애플리케이션이 있다면 파일 로깅을 쉽게 추가할 수 있습니다:

```rust
// 이전 (콘솔만)
env_logger::init();

// 이후 (콘솔 + 파일)
use dumq_mqtt::logging::init_simple_file_logging;
let _handle = init_simple_file_logging("app.log", log::LevelFilter::Debug).unwrap();
```

## 모범 사례

### 1. 적절한 로그 레벨 선택

- 상세한 디버깅 정보에는 `debug` 사용
- 일반적인 애플리케이션 흐름에는 `info` 사용
- 잠재적으로 문제가 될 수 있는 상황에는 `warn` 사용
- 주의가 필요한 실제 오류에는 `error` 사용

### 2. 로그 파일 구성

```rust
// 개발용
let _handle = init_simple_file_logging("debug.log", log::LevelFilter::Debug).unwrap();

// 프로덕션용
let _handle = init_file_logging("logs", "production.log", log::LevelFilter::Info).unwrap();
```

### 3. 환경 변수로 유연성 확보

```bash
# 개발
export RUST_LOG=debug

# 프로덕션
export RUST_LOG=info

# 문제 해결
export RUST_LOG=dumq_mqtt::server=debug,dumq_mqtt::client=warn
```

## 문제 해결

### 로그 파일이 생성되지 않음

1. 디렉토리가 존재하고 쓰기 가능한지 확인
2. 로그 문장보다 먼저 로깅 초기화가 호출되었는지 확인
3. 로그 레벨이 적절한지 확인

### 로그 출력이 너무 많음

1. 로그 레벨을 높이세요 (예: `debug`에서 `info`로)
2. 환경 변수를 사용하여 특정 모듈을 필터링
3. 더 많은 제어를 위해 `init_env_logging` 함수 사용

### 성능 문제

1. 프로덕션에서는 `info` 이상의 로그 레벨 사용
2. 로그 파일 크기 관리를 위해 파일 로테이션 고려
3. 디스크 공간 사용량 모니터링

## 자세한 문서

더 자세한 정보는 `docs/LOGGING.md` 파일을 참조하세요.
