import React, { useContext, useState } from "react";

import { ModelContext } from "../context/ModelProxy";
import { CharacterView } from "./Character/CharacterView";
import { MapView } from "./Map/MapView";
import { TabBar } from "./TabBar";
import { Toolbar } from "./Toolbar";

const enum MainViewType {
  Character,
  Map,
}

interface Props {
  type: MainViewType;
}

const MainView: React.FC<Props> = ({ type }) => {
  switch (type) {
    case MainViewType.Character:
      return <CharacterView />;
    case MainViewType.Map:
      return <MapView />;
    default:
      throw new Error("Unreachable - unsupported view type");
  }
};

const Proto: React.FC = () => {
  const { model } = useContext(ModelContext);
  const [mainViewType, setMainView] = useState(MainViewType.Map);
  const characterId = model.get_player_character_id();
  const character = model.get_character(characterId);

  const setMainViewGen = (type: MainViewType) => () => setMainView(type);

  try {
    return (
      <div>
        <TabBar>
          <button onClick={setMainViewGen(MainViewType.Character)}>
            Character
          </button>
          <button onClick={setMainViewGen(MainViewType.Map)}>Map</button>
          <Toolbar />
        </TabBar>
        <div>{character.copy_name()}</div>
        <MainView type={mainViewType} />
      </div>
    );
  } finally {
    character.free();
  }
};

export default Proto;
