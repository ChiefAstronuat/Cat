# Virtual Desktop Cat (가상 데스크톱 고양이) — 제품/기술 마스터 플랜

> 제품의 절대 목표는 하나입니다:  
> **"내 컴퓨터에서 귀여운 고양이를 키우는 경험"**

아래 문서는 위 목표를 달성하기 위해 필요한 항목만 **MECE(서로 겹치지 않고, 빠짐없이)** 구조로 정리한 실행 계획입니다.  
(우선순위: **귀여움 체감 > 안정적 동작 > 고품질 모델 > 경량 최적화 > 폭넓은 호환성**)

---

## 0. North Star 정의 (성공 기준 단일화)

### 0-1. 사용자 가치(What)
- 사용자는 앱을 켜는 즉시 "정서적으로 귀엽다"고 느낀다.
- 고양이는 살아있는 듯 반응하고, 반복 시에도 캐릭터가 일관된다.
- 방해되지 않는 선에서 존재감을 유지한다.

### 0-2. 제품 성공지표(How to measure)
- **Cuteness Retention**: 첫 7일 재실행률
- **Affinity Signal**: 쓰다듬기/클릭/반응 유도 상호작용 빈도
- **Annoyance Guard**: 강제 종료/숨김 전환율
- **Performance Safety**: 저사양 PC에서 프레임 급락/발열 이슈 재현률

---

## 1. 범위 정의 (MECE)

## 1-A. 반드시 해야 하는 것 (Core Scope)
1. 귀여운 외형 + 자연스러운 기본 행동
2. 사용자 입력에 대한 즉각 반응(마우스 중심)
3. 일관된 캐릭터성(같은 고양이 느낌 유지)
4. 저부하 백그라운드 동작
5. 울음/소리로 감정 전달

## 1-B. 나중에 해도 되는 것 (Extension Scope)
1. 랜덤 사물 상호작용
2. 다중 고양이
3. 고급 개인화/장기기억
4. 광고/유료화 고도화

## 1-C. 지금은 최소화/보류 (De-prioritized)
1. 복잡한 서버 인프라
2. 과도한 소셜 기능
3. 과한 실험적 생성 기능(안정성 저해)

---

## 2. 제품 경험 설계 (귀여움 중심)

## 2-A. 귀여움의 4요소
1. **실루엣**: 작은 머리 기울임, 꼬리 템포, 큰 눈 강조
2. **타이밍**: 반응 지연 100~300ms로 "살아있음" 표현
3. **변주**: 같은 행동도 미세 차이(속도/시선/귀 각도)
4. **애착 루프**: 이름 부르기, 쓰다듬기 반응, 가끔 먼저 관심 요청

## 2-B. 사용자 여정
1. 설치 후 첫 실행: 고양이 생성 + 이름 지정 + 기본 성격 선택
2. 일상 사용: 고양이가 화면을 걷고 쉬고 반응
3. 유대 강화: 사용 패턴을 학습해 성향 미세 조정
4. 장기 유지: 커스터마이징/행동팩 추가로 신선도 유지

---

## 3. 시스템 아키텍처 (기능별 분리)

```text
[Desktop Runtime]
  ├─ Overlay Renderer
  ├─ Animation Controller
  ├─ Interaction Listener
  ├─ Behavior Brain (FSM + Tiny Policy)
  ├─ Identity Manager (Cat DNA)
  ├─ Audio Engine (Meow Synth Hybrid)
  ├─ Performance Governor
  └─ Content/Monetization Switch (Optional)
```

### 3-1. Overlay Renderer
- 투명/항상 위 윈도우
- 클릭 투과 모드와 상호작용 모드 전환
- 화면 경계/멀티모니터 대응

### 3-2. Animation Controller
- 상태 기반 애니메이션 블렌딩
- 미세 랜덤 노이즈(귀, 꼬리, 눈 깜빡임)로 생동감 강화
- 원근 스케일과 이동 속도 동기화

### 3-3. Interaction Listener
- 마우스 이동/클릭/유휴 시간
- 활성 창 상태(집중모드/전체화면)
- 과도한 방해 방지를 위한 쿨다운 로직

### 3-4. Behavior Brain
- **Tier 1: FSM** (안정성)
- **Tier 2: Tiny Policy Model** (자연스러운 변주)
- **Tier 3: Rule Guard** (금지/안전/집중 보호)

### 3-5. Identity Manager
- CatID + Personality Vector + Visual Seed 고정
- 세션이 달라도 같은 고양이 정체성 유지

### 3-6. Audio Engine
- 이벤트 기반 울음 생성
- 저전력: 클립 기반 + DSP 변형
- 고품질 옵션: 경량 보코더(필요 시)

### 3-7. Performance Governor
- 렌더 주기와 AI 추론 주기 분리
- 유휴/집중/게임 모드별 동적 성능 정책
- CPU/RAM budget 초과 시 자동 저전력 모드

---

## 4. 행동 설계 (MECE 상태 구조)

## 4-A. 상위 상태(Emotion/Intent Layer)
- `Relaxed`
- `Curious`
- `Playful`
- `Sleepy`
- `Needy`

## 4-B. 하위 행동(Action Layer)
- 이동: 걷기/가벼운 달리기/멈춤
- 제스처: 앉기/그루밍/기지개/응시
- 소셜: 울음/부비기/시선 추적
- 휴식: 낮잠/하품/느린 꼬리 움직임

## 4-C. 전이 입력(Input Layer)
- 사용자 활동 강도
- 시간대(낮/밤)
- 최근 상호작용 히스토리
- 내부 변수(피로/호기심/관심욕구)

## 4-D. 전이 제약(Guard Layer)
- 동일 행동 과반복 제한
- 집중 모드 시 소리/개입 빈도 하향
- 갑작스러운 고강도 전이 금지

---

## 5. 원근감/귀여움 연출 (시각 일관성)

### 5-1. 스케일 규칙
```text
z = normalize(y)
scale = lerp(maxScale, minScale, z)
speed = baseSpeed * f(z)
```
- 화면 아래: 크게/가깝게
- 화면 위: 작게/멀게

### 5-2. 보조 연출
- 그림자 블러/투명도 = depth 연동
- 착지 시 1~2프레임 squash&stretch
- 시선 추적 시 머리 회전 제한각 적용(부자연 방지)

### 5-3. 사용자 옵션
- 원근감 강도 슬라이더
- 고정 크기 모드
- 저사양용 연출 단순화 토글

---

## 6. 외형 커스터마이징 (정체성 유지형)

### 6-1. 파라미터 영역 (Visual MECE)
1. 몸체: 체형/털 길이/꼬리 형태
2. 색상: 털색/무늬/눈색
3. 장식: 목걸이/리본/모자
4. 표정성: 눈매/입꼬리 강도

### 6-2. 저장 구조
- 사용자 프리셋(JSON)
- `appearance_seed` + `personality_seed` 동시 저장
- 버전 필드로 추후 호환성 관리

### 6-3. 일관성 원칙
- 생성 시 seed 고정
- 애니메이션 스켈레톤 규격 고정
- 음성 톤 임베딩 고정

---

## 7. AI 울음 설계 (현실적/경량 우선)

### 7-1. MVP (권장)
1. 울음 의도 분류: 반가움/요구/불만/졸림
2. 클립 선택 + pitch/formant/time-stretch
3. 최근 샘플 캐시 재사용

### 7-2. 고도화
1. 경량 토큰 생성기
2. 소형 보코더 on-device
3. CatID 조건부 음색 일관성 유지

### 7-3. 품질 체크
- 지연시간, 잡음, 반복 피로도
- 볼륨 자동 제한(업무 방해 방지)

---

## 8. 성능/호환성 목표 (핵심 비기능)

### 8-1. 타깃 지표
- Idle CPU: 1~3%
- Active CPU: 5% 이내 관리
- RAM: 150~300MB
- 기본 FPS: 30 (절전 15)

### 8-2. 최적화 전략
- 모델 양자화(int8/4bit)
- 이벤트 기반 업데이트
- 프레임 스킵/LOD
- 비가시/풀스크린 시 저전력 전환

### 8-3. 호환성 전략
- 1차: Windows 안정화
- 2차: macOS 확장
- 저사양 fallback preset 제공

---

## 9. 상품 품질 보증 체계

### 9-1. 품질 축
1. 귀여움 품질(애니메이션/표정/사운드)
2. 시스템 품질(크래시/메모리 누수/CPU 스파이크)
3. 모델 품질(행동 다양성/일관성)
4. 사용자 품질(방해 최소화)

### 9-2. 테스트 셋
- 장시간 러닝(4~8시간)
- 저사양 노트북 배터리 테스트
- 멀티모니터/고DPI
- 집중모드(게임/회의) 시나리오

---

## 10. 로드맵 (12주 실행안)

### Phase 1 (1~3주) — "귀여운 기본기"
- 오버레이 + 이동/충돌
- 기본 애니메이션 10종
- FSM 상태 전이

### Phase 2 (4~6주) — "키우는 느낌"
- 사용자 반응 입력 연결
- 원근감 스케일 + 시각 연출
- 외형 커스터마이징/프리셋

### Phase 3 (7~9주) — "AI 자연스러움"
- Tiny policy 결합
- 울음 하이브리드 엔진
- 성능 governor 고도화

### Phase 4 (10~12주) — "상품 완성도"
- 안정화/호환성 점검
- 설정 UX 개선
- (선택) 광고/유료 기능 on/off 플래그

---

## 11. 광고/유료화 (우선순위 낮음, 옵션)

> 본 프로젝트의 본질은 "귀여운 고양이를 키우는 경험"이므로,  
> 수익화 기능은 코어 경험을 해치지 않는 범위에서만 적용.

- 광고는 기본 비활성 권장(또는 명시적 opt-in)
- 빈도 상한 + 집중모드 차단 필수
- 유료 버전은 광고 제거 + 스킨/행동팩 중심

---

## 12. 리스크 및 대응

1. **귀여움 부족** → 실루엣/타이밍 튜닝 루프 강화
2. **행동 어색함** → FSM 제약 + 데이터 라벨 정제
3. **리소스 과다** → 추론 주기 하향 + 양자화 + 캐시
4. **호환성 이슈** → OS별 fallback 경로 사전 구축
5. **수익화 반감** → 코어 UX 우선, 수익화 최소 침습 원칙

---

## 13. 즉시 실행 체크리스트 (실무용)

- [ ] 타깃 OS 1순위 확정(권장: Windows)
- [ ] 행동 10종 + 상태전이도 확정
- [ ] CatID/Personality/Appearance JSON 스키마 정의
- [ ] 원근감 함수 + 그림자 규칙 구현
- [ ] 울음 MVP(의도 분류 + 클립 변형) 구현
- [ ] 성능 예산 초과 시 fallback 정책 구현
- [ ] 8시간 안정성 테스트 시나리오 작성

---

## 14. 요구 사항 명세서 (SRS Lite)

### 14-1. 문서 목적/범위
- 목적: "귀여운 고양이를 내 컴퓨터에서 키우는 경험"을 구현하기 위한 최소/필수 요구사항을 정의.
- 범위: 데스크톱 클라이언트 런타임(오버레이, 행동, 오디오, 커스터마이징, 성능 정책).
- 제외: 서버 운영, 소셜 기능, 고도 수익화 기능.

### 14-2. 이해관계자
- Product Owner: 귀여움 체감/제품 방향 의사결정
- Client Engineer: 렌더링/입력/시스템 통합
- AI Engineer: 행동 정책/오디오 추론 경량화
- QA: 성능/안정성/수용기준 검증

### 14-3. 기능 요구 사항 (FR)
- **FR-01 앱 라이프사이클**: 앱 실행 후 3초 이내 고양이 오버레이를 표시하고 종료 시 리소스를 정상 해제한다.
- **FR-02 오버레이 렌더링**: 항상 위 투명 창, 클릭-투과/상호작용 모드를 제공한다.
- **FR-03 이동/충돌**: 화면 경계 충돌 시 반사/전환 규칙에 따라 자연스럽게 이동한다.
- **FR-04 행동 상태기계**: 최소 10개 행동(Idle/Walk/Run/Sit/Groom/Nap/Stretch/Look/Meow/Play)을 지원한다.
- **FR-05 사용자 반응**: 마우스 접근/클릭/유휴 이벤트에 300ms 이내 반응한다.
- **FR-06 원근감 옵션**: 자동 원근감(깊이 기반 scale)과 고정 크기 모드를 전환 가능해야 한다.
- **FR-07 외형 커스터마이징**: 털색/눈색/무늬/액세서리 변경 및 프리셋 저장·복원을 지원한다.
- **FR-08 정체성 일관성**: CatID, personality_seed, appearance_seed를 기준으로 세션 간 캐릭터성을 유지한다.
- **FR-09 울음 생성**: 상태 기반 울음 의도 선택 + 클립 변형(pitch/formant/stretch)을 제공한다.
- **FR-10 저전력 정책**: 유휴/집중 모드에서 FPS/추론주기를 자동 하향한다.
- **FR-11 설정 저장소**: 사용자 설정과 프리셋을 사용자 디렉터리에 영속 저장한다.
- **FR-12 오류 복원력**: 모델 로드 실패 시 fallback(규칙 기반 행동+기본 사운드)으로 동작을 지속한다.

### 14-4. 비기능 요구 사항 (NFR)
- **NFR-01 성능**: Idle CPU 1~3%, Active CPU 5% 이내, RAM 150~300MB 목표.
- **NFR-02 프레임 안정성**: 기본 모드 30FPS, 저전력 15FPS 하한.
- **NFR-03 안정성**: 8시간 연속 실행 중 치명적 크래시 0회.
- **NFR-04 호환성**: 1차 Windows 10/11, 2차 macOS 14+.
- **NFR-05 반응성**: 입력 이벤트->반응 애니메이션 시작 지연 300ms 이하.
- **NFR-06 일관성**: 동일 seed로 재실행 시 외형/성격 편차가 허용 범위 내.
- **NFR-07 사용성**: 초회 실행 온보딩 1분 내 완료.

### 14-5. 데이터/설정 스키마 요구
```json
{
  "cat_id": "uuid",
  "name": "Momo",
  "appearance_seed": 12345,
  "personality_seed": 98765,
  "appearance": {
    "fur": "tabby_orange",
    "eye": "emerald",
    "accessory": "red_collar"
  },
  "runtime": {
    "perspective_mode": "auto",
    "fps_target": 30,
    "low_power": false
  }
}
```

### 14-6. 수용 기준 (AC)
- **AC-01**: 첫 실행 후 고양이 생성/표시까지 3초 이내.
- **AC-02**: 30분 사용 중 최소 20회 이상 자연 반응(시선/꼬리/행동 전이) 발생.
- **AC-03**: 커스터마이징 저장 후 재실행 시 동일 외형 100% 복원.
- **AC-04**: 집중모드에서 울음 빈도와 적극 행동 빈도가 기본 대비 감소.
- **AC-05**: 저전력 프로파일에서 15FPS 미만 지속 구간이 5초를 넘지 않음.
- **AC-06**: 모델 비가용 상황에서도 앱이 종료되지 않고 fallback 동작.

### 14-7. 추적성 매트릭스 (요구사항 ↔ 검증)
- FR-01/02/11 → 실행/종료/재실행 시나리오 테스트
- FR-03/04/05/06 → 상호작용 시뮬레이션 + 상태전이 로그 검증
- FR-07/08 → 프리셋 저장/로드 및 seed 재현성 테스트
- FR-09/10/12 → 오디오 이벤트 테스트 + 저전력 모드 + fallback 테스트
- NFR-01~07 → 장시간/저사양/반응성/온보딩 측정 테스트

---

## 15. 구현 순서 (우선순위/의존성 기반)

### 15-1. 구현 원칙
1. 귀여움 체감에 직접 기여하는 기능 먼저.
2. 안정성/성능 안전장치를 동시 구축.
3. AI는 "기본 동작 완성 후" 단계적으로 결합.

### 15-2. 단계별 구현 순서 (권장)

#### Step 0 — 프로젝트 골격/환경 (1주)
- 저장소 구조/빌드 파이프라인/설정 파일 체계 확정
- 오버레이 실행 샘플/로그 수집기/기본 설정 저장 구현
- 산출물: 실행 가능한 빈 껍데기 앱 + 진단 로그

#### Step 1 — 코어 렌더링/이동 (1~2주)
- 스프라이트 렌더링, 경계 충돌, 기본 이동
- 클릭-투과/상호작용 모드 전환
- 산출물: 화면 위에서 자연스럽게 돌아다니는 고양이

#### Step 2 — FSM 행동 엔진 (1~2주)
- 행동 10종 + 상태전이 표 구현
- 반복 방지/쿨다운/가드룰 적용
- 산출물: 단조롭지 않은 기본 행동 루프

#### Step 3 — 상호작용/귀여움 연출 (1주)
- 마우스 접근/클릭 반응
- 미세 애니메이션(눈깜빡임, 꼬리 템포, 머리 기울임)
- 산출물: "살아있음" 체감 확보

#### Step 4 — 원근감 + 커스터마이징 (1~2주)
- 깊이 기반 scale + 그림자 연동
- 외형 파라미터 편집, 프리셋 저장/복원
- 산출물: 키우는 느낌 + 개성 부여

#### Step 5 — 울음 MVP + 오디오 제어 (1주)
- 의도 기반 울음 선택 + DSP 변형
- 볼륨 제한, 집중모드 음소거 정책
- 산출물: 감정 전달 가능한 사운드

#### Step 6 — 성능 거버너 + fallback (1주)
- 저전력 프로파일, FPS/추론 주기 동적 조절
- 모델 실패 fallback 경로 구현
- 산출물: 저사양 안정 구동

#### Step 7 — Tiny Policy 모델 결합 (1~2주)
- FSM 위 정책 보조 추론 연결
- 정책 스위치/실험 플래그/로그 비교
- 산출물: 자연스러운 행동 변주 향상

#### Step 8 — QA 하드닝/릴리스 준비 (1주)
- 8시간 soak test, 멀티모니터, 고DPI, 집중모드 검증
- 설치패키지/자동시작/설정 마이그레이션 점검
- 산출물: 배포 가능한 MVP 빌드

### 15-3. 선후행 의존성
- Step 1 선행 없이는 Step 2~7 진행 불가
- Step 2 완료 후 Step 3/5 병렬 가능
- Step 4는 Step 1 완료 후 독립 진행 가능
- Step 6은 Step 2/5 이후 적용 시 효과 극대화
- Step 8은 모든 단계 완료 후 필수 게이트

### 15-4. 완료 정의 (Definition of Done)
- 핵심 FR(01~10) 구현 및 AC-01~06 통과
- NFR 성능/안정성 목표를 목표치 범위 내 충족
- 최소 2개 환경(표준/저사양)에서 재현성 확인

---

## 16. 설치 환경 구성 (개발/실행 환경)

### 16-1. 권장 개발 환경
- OS: Windows 11 (우선), macOS 14+
- CPU: 4코어 이상
- RAM: 16GB 이상(개발), 8GB(실행 최소)
- 저장공간: 5GB 이상

### 16-2. 필수 런타임/도구
- Git
- Rust toolchain (`rustup`, stable)
- Node.js 20+ / pnpm or npm (Tauri 프론트 사용 시)
- Python 3.10+ (데이터 전처리/모델 변환 스크립트)
- ONNX Runtime (로컬 추론)

### 16-3. 저장소 초기 설정 절차
1. 저장소 클론
2. Rust/Node/Python 버전 확인
3. 의존성 설치
4. 개발 모드 실행
5. 기본 품질 체크 실행

### 16-4. 예시 명령어 (Tauri 기준)
```bash
git clone <repo_url>
cd Cat

# Rust
rustup toolchain install stable
rustup default stable

# Node
node -v
npm -v

# Front/desktop deps
npm install

# Dev run
npm run tauri dev

# Build
npm run tauri build
```

### 16-5. Python/모델 유틸 환경 (선택)
```bash
python -m venv .venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate
pip install -U pip
pip install onnx onnxruntime numpy scipy librosa
```

### 16-6. 환경 변수 권장
```bash
# 앱 모드
APP_ENV=development

# 성능/디버그
CAT_DEBUG_OVERLAY=1
CAT_LOW_POWER_DEFAULT=0
CAT_FPS_TARGET=30

# 모델 경로
CAT_POLICY_MODEL_PATH=./models/policy.onnx
CAT_AUDIO_MODEL_PATH=./models/meow.onnx
```

### 16-7. 실행 환경 프로파일
- **Profile A (저사양)**: 15 FPS, AI 1~2Hz, 오디오 단순 모드
- **Profile B (기본)**: 30 FPS, AI 2~5Hz, 일반 효과
- **Profile C (고품질)**: 60 FPS(옵션), 고급 시각/오디오 효과

### 16-8. 배포 패키징 가이드
- Windows: MSI/EXE 설치파일 + 자동 시작 옵션
- macOS: DMG + 접근성/입력 권한 안내
- 설정/프리셋은 사용자 디렉토리에 저장(업데이트 시 보존)

---

## 17. 한 줄 결론

이 프로젝트는 "고양이를 만든다"가 아니라, **"사용자가 내 컴퓨터에서 진짜 귀여운 고양이를 키운다고 느끼게 하는 경험"**을 만드는 일입니다.  
모든 기술 선택은 그 목표(귀여움 체감 + 안정적 동작 + 상품 품질)를 위해서만 존재해야 합니다.

---

## 18. 4개 에이전트 작업 분배안 (병렬 실행)

> 목표: 구현 초기부터 병렬 처리로 속도를 높이되, 최종 사용자 가치(귀여움 체감)를 가장 먼저 확보한다.

### 18-1. Agent A — 클라이언트 코어/오버레이
- 담당 범위: `Overlay Renderer`, `Animation Controller` 기본 루프
- 핵심 작업:
  1. 투명 항상-위 윈도우 + 클릭-투과/상호작용 모드 전환
  2. 스프라이트 렌더링, 화면 경계 충돌, 기본 이동
  3. 30FPS/15FPS 전환 훅 제공(성능 거버너 연동용)
- 완료 기준(DoD): 고양이가 데스크톱 위에서 자연 이동, 모드 전환 시 입력 충돌 없음

### 18-2. Agent B — 행동 엔진/FSM
- 담당 범위: `Behavior Brain` 중 FSM + Guard Layer
- 핵심 작업:
  1. 행동 10종 상태 정의 및 전이 테이블 구현
  2. 쿨다운/반복 억제/집중모드 감쇠 가드룰 적용
  3. 입력 이벤트(마우스 접근/클릭/유휴) 처리 인터페이스 정의
- 완료 기준(DoD): 30분 자동 실행에서 상태 루프가 단조롭지 않고 규칙 위반 전이 없음

### 18-3. Agent C — 정체성/커스터마이징/데이터
- 담당 범위: `Identity Manager`, `settings-store`, 프리셋 스키마
- 핵심 작업:
  1. `cat_id`, `appearance_seed`, `personality_seed` 생성/저장/로드
  2. 외형 파라미터(털색/눈색/무늬/액세서리) 스키마 및 버전 관리
  3. 재실행 시 100% 동일 복원 검증 스크립트 작성
- 완료 기준(DoD): 프리셋 저장 후 재실행 시 외형/성격 일관성 유지

### 18-4. Agent D — 오디오/성능 거버너/검증 자동화
- 담당 범위: `Audio Engine`, `Performance Governor`, 기본 QA 자동화
- 핵심 작업:
  1. 울음 의도(반가움/요구/불만/졸림)→클립 선택 + DSP 변형
  2. 저전력 정책(유휴/집중 모드에서 FPS/추론 주기 하향)
  3. 8시간 soak test 및 CPU/RAM 로깅 스크립트 구성
- 완료 기준(DoD): 저전력 모드에서 안정 동작, 모델 실패 시 fallback 유지

### 18-5. 통합 순서/의존성
1. **주 1~2**: Agent A, B 병렬 착수 (렌더 기반 + FSM 기반 동시 확보)
2. **주 2~3**: Agent C가 A/B 산출물과 연결해 정체성/프리셋 고정
3. **주 3~4**: Agent D가 오디오/성능 거버너 결합, 자동화 측정 파이프라인 구축
4. **주 4 말**: 통합 QA(AC-01~06 중심) + 릴리스 후보 빌드

### 18-6. 공통 인터페이스 계약 (충돌 방지)
- 이벤트 버스: `UserInputEvent`, `SystemModeEvent`, `BehaviorCommand`
- 상태 스냅샷: `CatRuntimeState` (행동/위치/스케일/성능모드)
- 설정 계약: `cat_profile.json` 단일 진실 소스(SSOT)
- 머지 규칙: 기능 브랜치 단위 PR, 하루 1회 통합 브랜치 리베이스

---

## 19. Agent A 수행안 (클라이언트 코어/오버레이 실행)

> 요청사항인 "A 수행"에 맞춰, Agent A가 바로 구현 가능한 단위로 작업을 고정한다.

### 19-1. 목표 (A의 Definition of Success)
- 앱 실행 후 3초 이내 투명 오버레이가 나타난다.
- 고양이 스프라이트가 화면 경계를 넘지 않고 자연스럽게 이동한다.
- 클릭-투과 모드와 상호작용 모드를 단축키/설정으로 즉시 전환할 수 있다.
- 렌더 루프가 30FPS(기본)/15FPS(저전력)로 전환 가능하며 끊김이 누적되지 않는다.

### 19-2. 구현 범위 (In-Scope / Out-of-Scope)
- In-Scope
  1. 오버레이 윈도우 생성/제어
  2. 스프라이트 렌더링 루프 + 위치 업데이트
  3. 경계 충돌/반사 규칙
  4. FPS 프로파일 훅(`quality=normal|low_power`) 제공
- Out-of-Scope
  1. 행동 전이 로직(FSM)은 Agent B 소관
  2. 커스터마이징 저장/로드는 Agent C 소관
  3. 오디오/성능 정책 판정은 Agent D 소관

### 19-3. 작업 분해 (WBS)
1. **Window Layer**
   - 투명/항상 위/작업표시줄 비노출 옵션 적용
   - 클릭-투과 토글 API: `setInteractionMode(passThrough: boolean)`
2. **Render Layer**
   - 스프라이트 atlas 로더 + 애니메이션 프레임 타이머
   - delta-time 기반 위치 업데이트
3. **Motion Layer**
   - 속도 벡터(`vx`, `vy`) 기반 이동
   - 화면 경계 충돌 시 반사 + 감쇠 계수 적용(자연스러움)
4. **Profile Hook Layer**
   - `setRenderProfile(normal|low_power)` 구현
   - `normal=30FPS`, `low_power=15FPS` 프레임 캡 제공
5. **Telemetry/Debug Layer**
   - FPS, frameTime, dropFrame count 로깅
   - 디버그 오버레이 토글(`CAT_DEBUG_OVERLAY=1`)

### 19-4. Agent A 인터페이스 계약 (타 에이전트 연동점)
- 입력(consume)
  - `BehaviorCommand.MoveTo(x, y, speed)`
  - `SystemModeEvent.FocusModeChanged(isFocus)`
  - `SystemModeEvent.PowerProfileChanged(profile)`
- 출력(produce)
  - `CatRuntimeState.position`
  - `CatRuntimeState.scale`
  - `CatRuntimeState.renderStats` (`fps`, `frameTimeMs`, `dropFrames`)

### 19-5. 완료 기준(DoD) — 측정 가능한 조건
1. 5회 연속 실행에서 오버레이 초기 표시 시간 3초 이내 100% 충족
2. 30분 자동 이동 중 화면 이탈 0회
3. 모드 전환 100회 반복 시 클릭 충돌/입력 먹통 0회
4. 저전력 프로파일 전환 후 평균 FPS 15±2 유지
5. 치명 오류(crash/freeze) 0회

### 19-6. 테스트 방법 (A 전용)
- 기능 테스트
  - 오버레이 생성/종료 반복 20회
  - 클릭-투과 ON/OFF 전환 후 데스크톱 아이콘 클릭 가능 여부 확인
- 동작 테스트
  - 코너 4지점 강제 이동 시 반사 규칙 정상 여부 확인
  - 창 크기/해상도 변경 후 좌표 보정 확인
- 성능 테스트
  - `normal` 10분, `low_power` 10분 실행 후 FPS 로그 비교
  - drop frame 비율 1% 이하 목표

### 19-7. 우선 구현 순서 (A 내부)
1. Window Layer
2. Render Layer(정적 스프라이트)
3. Motion Layer(이동/경계)
4. Interaction Mode 전환
5. Render Profile 훅 + 로깅
6. Agent B/D 연동용 이벤트 입출력 연결

---

## 20. Agent B 수행안 (행동 엔진/FSM 실행)

> 요청사항인 "B 수행"에 맞춰, Agent B가 행동 엔진을 독립적으로 구현/검증할 수 있도록 실행 계획을 확정한다.

### 20-1. 목표 (B의 Definition of Success)
- FSM 10개 상태가 정의되고, 상태 전이가 규칙 위반 없이 동작한다.
- 사용자 입력(마우스 접근/클릭/유휴)에 대해 300ms 이내 행동 전이 신호를 생성한다.
- 30분 자동 실행에서 행동 패턴이 과도하게 반복되지 않는다.
- 집중모드/저전력 모드에서 개입도(소리/활동성) 감쇠가 적용된다.

### 20-2. 구현 범위 (In-Scope / Out-of-Scope)
- In-Scope
  1. FSM 상태/전이 테이블 구현
  2. Guard Layer(쿨다운/반복 억제/급격 전이 방지)
  3. 입력 이벤트 해석기(UserInput -> Intent) 구현
  4. BehaviorCommand 생성기(렌더/오디오 연동용)
- Out-of-Scope
  1. 오버레이 렌더링/좌표 업데이트는 Agent A 소관
  2. seed 저장/복원 및 프리셋 관리는 Agent C 소관
  3. 오디오 합성/성능 governor 운영은 Agent D 소관

### 20-3. 상태 모델 설계 (MECE)
1. **상위 상태 (Intent)**
   - `Relaxed`, `Curious`, `Playful`, `Sleepy`, `Needy`
2. **하위 행동 (Action)**
   - `Idle`, `Walk`, `Run`, `Sit`, `Groom`, `Nap`, `Stretch`, `Look`, `Meow`, `Play`
3. **전이 입력 (Input Signals)**
   - `cursor_distance`, `click_burst`, `idle_seconds`, `time_of_day`, `focus_mode`
4. **가드 규칙 (Guards)**
   - 동일 행동 연속 횟수 제한
   - 고강도 행동(`Run`, `Play`) 최소 쿨다운
   - 집중모드 시 `Meow` 빈도/강도 감쇠

### 20-4. 작업 분해 (WBS)
1. **State Definition Layer**
   - 상태 enum/전이 조건/우선순위 정의
2. **Transition Engine Layer**
   - `evaluateTransition(currentState, signals, memory)` 구현
   - 확률 전이 + deterministic guard 결합
3. **Behavior Memory Layer**
   - 최근 N개 행동 히스토리/쿨다운 타임스탬프 저장
   - 반복 패널티 점수 계산
4. **Event Interpreter Layer**
   - 입력 이벤트를 정규화 신호로 변환
   - 사용자 활동 강도 산출(`engagement_score`)
5. **Command Emitter Layer**
   - `BehaviorCommand` 발행(`MoveTo`, `PlayAnim`, `TriggerMeow`)
   - 상태 스냅샷(`CatRuntimeState.behavior`) 갱신

### 20-5. Agent B 인터페이스 계약 (타 에이전트 연동점)
- 입력(consume)
  - `UserInputEvent.MouseMove(x, y, speed)`
  - `UserInputEvent.MouseClick(x, y, button)`
  - `UserInputEvent.IdleChanged(idleSeconds)`
  - `SystemModeEvent.FocusModeChanged(isFocus)`
- 출력(produce)
  - `BehaviorCommand.MoveTo(x, y, speed)`
  - `BehaviorCommand.PlayAnim(action, durationMs)`
  - `BehaviorCommand.TriggerMeow(intent, intensity)`
  - `CatRuntimeState.behavior` (`intent`, `action`, `cooldowns`, `engagementScore`)

### 20-6. 완료 기준(DoD) — 측정 가능한 조건
1. 상태 10종 + 전이 규칙 문서/코드 일치율 100%
2. 30분 시뮬레이션에서 단일 행동 점유율 35% 이하
3. 입력 이벤트 후 행동 전이 명령 발행까지 p95 300ms 이하
4. 집중모드에서 `Meow` 트리거 빈도 기본 대비 50% 이상 감소
5. Guard 규칙 위반 전이(쿨다운 미준수/급격 전이) 0건

### 20-7. 테스트 방법 (B 전용)
- 단위 테스트
  - 전이 함수 조건별 테스트(정상/경계/예외)
  - 쿨다운/반복 패널티 계산 검증
- 시뮬레이션 테스트
  - 30분 가상 입력 스트림 재생 후 행동 분포 확인
  - 무입력(Idle) 장기 구간에서 Sleepy 전이 안정성 확인
- 통합 전 테스트
  - CommandEmitter 목(mock)으로 A/D 연동 포맷 검증
  - 로그 기반 p95 반응 지연 계산

### 20-8. 우선 구현 순서 (B 내부)
1. State Definition Layer
2. Transition Engine Layer
3. Guard/Memory Layer
4. Event Interpreter Layer
5. Command Emitter Layer
6. 시뮬레이션 기반 튜닝 + DoD 검증

---

## 21. Agent C 수행안 (정체성/커스터마이징/데이터 실행)

> 요청사항인 "C 수행"에 맞춰, Agent C가 정체성 일관성과 사용자 커스터마이징을 독립적으로 구현/검증할 수 있도록 실행 계획을 확정한다.

### 21-1. 목표 (C의 Definition of Success)
- `cat_id`, `appearance_seed`, `personality_seed`가 안정적으로 생성/저장/복원된다.
- 외형 프리셋(털색/눈색/무늬/액세서리) 변경이 즉시 반영되고 재실행 후 동일하게 복원된다.
- 스키마 버전 관리로 향후 필드 확장 시에도 하위 호환이 보장된다.
- 잘못된 설정 파일(누락/타입 오류/구버전)에서도 안전한 fallback으로 실행이 지속된다.

### 21-2. 구현 범위 (In-Scope / Out-of-Scope)
- In-Scope
  1. 정체성 데이터 모델(`CatProfile`) 정의
  2. 설정 저장소(read/write/migrate/validate) 구현
  3. 커스터마이징 파라미터 스키마 및 프리셋 관리
  4. 복원 재현성 검증 스크립트/테스트 케이스 작성
- Out-of-Scope
  1. 오버레이 렌더링/입력 처리 루프는 Agent A 소관
  2. 행동 전이 및 Guard 판단은 Agent B 소관
  3. 오디오 합성/성능 모드 판정은 Agent D 소관

### 21-3. 데이터 모델/스키마 설계
1. **Identity Layer**
   - `cat_id: uuid`
   - `appearance_seed: int`
   - `personality_seed: int`
2. **Appearance Layer**
   - `body_type`, `fur_pattern`, `fur_color`, `eye_color`, `accessory`
3. **Runtime Preference Layer**
   - `perspective_mode`, `fps_target`, `low_power_default`, `audio_volume`
4. **Metadata Layer**
   - `schema_version`, `created_at`, `updated_at`, `profile_name`

### 21-4. 작업 분해 (WBS)
1. **Schema Definition Layer**
   - JSON Schema 초안/검증 규칙 정의
   - 필수/선택 필드, 기본값 전략 수립
2. **Storage Layer**
   - 사용자 디렉터리 저장 경로 확정
   - 원자적 저장(임시 파일 -> rename)으로 손상 방지
3. **Migration Layer**
   - `v1 -> v2` 등 버전 업 변환기 구현
   - 누락 필드 보정/폐기 필드 호환 처리
4. **Validation/Fallback Layer**
   - 스키마 검증 실패 시 안전 기본 프로필 로드
   - 오류 로그/복구 사유 기록
5. **Preset Management Layer**
   - preset 저장/불러오기/복제/삭제 API
   - 현재 활성 프리셋 포인터 관리

### 21-5. Agent C 인터페이스 계약 (타 에이전트 연동점)
- 입력(consume)
  - `CustomizationEvent.UpdateAppearance(patch)`
  - `SystemModeEvent.ProfileRequested(profileName)`
  - `SystemModeEvent.AppStarted(appVersion)`
- 출력(produce)
  - `CatProfileLoaded(profile)`
  - `CatProfileUpdated(profile, revision)`
  - `CatRuntimeState.identity` (`catId`, `appearanceSeed`, `personalitySeed`, `profileName`)

### 21-6. 완료 기준(DoD) — 측정 가능한 조건
1. 저장->재실행->복원 시 identity 필드 일치율 100%
2. 프리셋 저장/로드 100회 반복에서 데이터 손상 0건
3. 스키마 검증 실패 케이스 20종에서 앱 비정상 종료 0회
4. 구버전 프로필 마이그레이션 성공률 100%
5. 프로필 저장 연산 p95 50ms 이하(로컬 디스크 기준)

### 21-7. 테스트 방법 (C 전용)
- 단위 테스트
  - schema validator 정상/비정상 케이스
  - migration 함수 버전별 변환 검증
- 내구성 테스트
  - 저장 중 강제 중단 시나리오 후 파일 무결성 확인
  - 100회 연속 저장/로드 반복
- 회귀 테스트
  - 샘플 구버전 프로필 세트 재생 후 동일 출력 확인
  - fallback 기본 프로필 로드 경로 검증

### 21-8. 우선 구현 순서 (C 내부)
1. Schema Definition Layer
2. Storage Layer
3. Validation/Fallback Layer
4. Migration Layer
5. Preset Management Layer
6. 재현성/내구성 테스트 + DoD 검증

---

## 22. Agent D 수행안 (오디오/성능 거버너/검증 자동화 실행)

> 요청사항인 "D 수행"에 맞춰, Agent D가 오디오 품질·저전력 정책·장시간 안정성을 독립적으로 구현/검증할 수 있도록 실행 계획을 확정한다.

### 22-1. 목표 (D의 Definition of Success)
- 울음 의도 기반 오디오 출력이 상태 문맥에 맞게 재생되고, 과도한 반복/볼륨 피로를 억제한다.
- 성능 거버너가 `normal`/`low_power`/`focus_mode`에서 FPS·추론 주기를 자동 조절한다.
- 8시간 soak test 동안 치명 오류 없이 CPU/RAM/FPS 로그가 안정 범위를 유지한다.
- 정책/모델 비가용 시 fallback 오디오·fallback 성능 프로파일로 서비스가 지속된다.

### 22-2. 구현 범위 (In-Scope / Out-of-Scope)
- In-Scope
  1. 오디오 의도 매핑 + 클립 선택 + DSP 변형 체인
  2. 성능 거버너 정책 엔진(모드 감지/프로파일 전환)
  3. 런타임 지표 수집기(CPU/RAM/FPS/dropFrame)
  4. soak test/리포트 자동화 스크립트
- Out-of-Scope
  1. 오버레이 윈도우/렌더 루프 구현은 Agent A 소관
  2. 행동 상태 전이 로직은 Agent B 소관
  3. 프로필 스키마 저장/마이그레이션은 Agent C 소관

### 22-3. 오디오 설계 (MVP 우선)
1. **Intent Mapping Layer**
   - 입력: `intent`, `action`, `intensity`, `focus_mode`
   - 출력: `meow_type`, `volume`, `dsp_profile`
2. **Clip Selector Layer**
   - 동일 타입 연속 재생 제한(anti-repetition window)
   - 최근 재생 이력 기반 확률 재가중
3. **DSP Chain Layer**
   - `pitch_shift`, `formant_shift`, `time_stretch`, `limiter`
   - 볼륨 상한 및 피크 보호
4. **Audio Fallback Layer**
   - 모델/클립 불가 시 기본 meow bank 사용
   - 오류 발생 시 무음 대신 저자극 기본음 재생

### 22-4. 성능 거버너 설계
1. **Mode Detector**
   - `idle`, `interactive`, `focus_mode`, `fullscreen`
2. **Policy Table**
   - `normal`: 30FPS, AI 2~5Hz
   - `low_power`: 15FPS, AI 1~2Hz
   - `focus_mode`: 15FPS, meow 빈도 감쇠, 효과 최소화
3. **Budget Enforcer**
   - CPU/RAM 임계치 초과 시 단계적 하향(soft clamp -> hard clamp)
4. **Recovery Controller**
   - 임계치 복귀 시 점진적 성능 복원(급격 변동 방지)

### 22-5. 작업 분해 (WBS)
1. **Audio Intent/Selector Layer** 구현
2. **DSP/Fallback Layer** 구현
3. **Governor Mode/Policy Layer** 구현
4. **Metrics Collector Layer** 구현
5. **Soak Test Harness Layer** 구현
6. **자동 리포트 생성기(요약/경고/회귀 비교)** 구현

### 22-6. Agent D 인터페이스 계약 (타 에이전트 연동점)
- 입력(consume)
  - `BehaviorCommand.TriggerMeow(intent, intensity)`
  - `SystemModeEvent.FocusModeChanged(isFocus)`
  - `SystemModeEvent.PowerProfileChanged(profile)`
  - `CatRuntimeState.renderStats` (`fps`, `frameTimeMs`, `dropFrames`)
- 출력(produce)
  - `PerformanceProfileApplied(profile, reason)`
  - `AudioPlaybackState(lastClip, gain, dspProfile)`
  - `RuntimeHealthSnapshot(cpuPct, ramMb, fps, aiHz, errorCount)`

### 22-7. 완료 기준(DoD) — 측정 가능한 조건
1. 동일 meow 타입 3회 초과 연속 재생률 1% 이하
2. focus_mode 진입 후 meow 트리거 빈도 기본 대비 50% 이상 감소
3. 8시간 soak test에서 crash/freeze 0회
4. Idle CPU 1~3%, Active CPU 5% 이내(목표), RAM 150~300MB 범위 유지
5. 임계치 초과 이벤트 발생 시 2초 이내 profile downshift 적용
6. fallback 경로 테스트 10종에서 서비스 중단 0건

### 22-8. 테스트 방법 (D 전용)
- 오디오 테스트
  - intent별 샘플 100회 재생 분포 검증
  - 피크/클리핑 탐지 및 limiter 동작 확인
- 성능 테스트
  - 모드 전환 시 FPS/AI Hz 정책 적용 여부 검증
  - CPU/RAM 부하 주입 시 downshift/recovery 동작 검증
- 안정성 테스트
  - 8시간 soak 자동화 + 30분 간격 health snapshot 기록
  - 장애 주입(클립 누락/모델 경로 오류) 후 fallback 지속성 확인

### 22-9. 우선 구현 순서 (D 내부)
1. Governor Mode/Policy Layer
2. Metrics Collector Layer
3. Audio Intent/Selector + DSP/Fallback Layer
4. Soak Test Harness + 리포트 자동화
5. 장애 주입 테스트/임계치 튜닝
6. A/B/C 통합 후 회귀 검증
