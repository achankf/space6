import React, { createContext, useCallback, useState } from "react";

import type { Model } from "../../pkg";

interface ContextData {
  model: Model;
  updateTick: () => void;
  signal: unknown;

  // global game data constants
  numUniverses: number;
  universeNames: string[];
}

export const ModelContext = createContext<ContextData>({} as ContextData);

export const ModelProvider: React.FC<{ model: Model }> = ({
  model,
  children,
}) => {
  const [signal, setSignal] = useState({});

  const updateTick = useCallback(() => {
    model.progress_game();
    setSignal({});
  }, []);

  const numUniverses = model.count_universes();

  const universeNames = (() => {
    const ret = [];
    for (let i = 0; i < numUniverses; i++) {
      ret.push(model.get_universe_name(i));
    }
    return ret;
  })();

  const value: ContextData = {
    model,
    updateTick,
    signal,
    numUniverses,
    universeNames,
  };

  const { Provider } = ModelContext;

  return <Provider value={value}>{children}</Provider>;
};
