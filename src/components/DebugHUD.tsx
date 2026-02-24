import React from 'react';

const DebugHUD: React.FC = () => {
    // CAT_DEBUG_OVERLAY=1 일때 활성화되는 디버그 패널
    return (
        <div className="debug-hud">
            <p>FPS: {`--`}</p>
            <p>State: {`IDLE`}</p>
            <p>Pos: (x:0, y:0, z:0)</p>
        </div>
    );
};

export default DebugHUD;
