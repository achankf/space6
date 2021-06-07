import React, { useContext, useState } from "react";

import { ModelContext } from "../context/ModelProxy";

const TICK_PER_SECOND = 10;
const PERIOD_PER_TICK = 1000 / TICK_PER_SECOND; // period over 1 second

export const Toolbar: React.FC = () => {
  const [gameLoopHandle, setGameLoopHandle] = useState<number | undefined>(
    undefined
  );
  const { updateTick } = useContext(ModelContext);

  return (
    <div>
      <button
        disabled={!!gameLoopHandle}
        onClick={() => {
          const handle = window.setInterval(updateTick, PERIOD_PER_TICK);
          setGameLoopHandle(handle);
        }}
      >
        Start
      </button>
      <button
        disabled={!gameLoopHandle}
        onClick={() => {
          window.clearInterval(gameLoopHandle);
          setGameLoopHandle(undefined);
        }}
      >
        Stop
      </button>
    </div>
  );
};
