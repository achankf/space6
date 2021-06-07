import React, { ReactElement, useContext } from "react";

import { ModelContext } from "../../context/ModelProxy";

interface Character {
  id: number;
  name: string;
  coor: unknown;
}

export const CharacterView: React.FC = () => {
  const { model } = useContext(ModelContext);

  const characters = model.get_all_characters();
  console.log("HIHI");

  const list = (() => {
    const len = characters.len();
    const ret = [];
    for (let i = 0; i < len; ++i) {
      const character = characters.at(i);
      const name = character.copy_name();
      const coor = character.copy_coor();
      try {
        ret.push(
          <div key={i}>
            name: {name}, coor: {JSON.stringify(coor)}
          </div>
        );
      } finally {
        character.free();
      }
    }
    return ret;
  })();

  return (
    <div>
      Characters
      {list}
    </div>
  );
};
