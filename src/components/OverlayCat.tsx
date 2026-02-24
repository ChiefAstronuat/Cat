import React, { useEffect, useRef } from 'react';
import { AnimationCtrl } from '../controllers/AnimationCtrl';

const OverlayCat: React.FC = () => {
    const catRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        // 렌더링 루프 및 상태 연동 초기화
        const animCtrl = new AnimationCtrl();
        console.log("OverlayCat Mounted. Waiting for interactions...");
    }, []);

    return (
        <div ref={catRef} className="overlay-cat">
            <div className="cat-sprite idle" />
        </div>
    );
};

export default OverlayCat;
