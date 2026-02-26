// [5-1] 화면 좌표(z)에 맞춘 원근감 스케일/속도 변형 로직

export class PerspectiveCtrl {
    static getScaleForDepth(y: number, screenHeight: number) {
        // z = normalize(y); scale = lerp(maxScale, minScale, z)
        return 1.0;
    }
}
