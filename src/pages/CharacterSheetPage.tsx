// src/pages/CharacterSheetPage.tsx
import React from 'react';

interface CharacterSheetProps {
  tomlData: Record<string, any>;
}

const CharacterSheetPage: React.FC<CharacterSheetProps> = ({ tomlData }) => {
  return (
    <div className="character-sheet">
      <h1>Character Sheet</h1>
      <div>
        <label>Character Name: </label>
        <span>{tomlData.info.charname}</span>
      </div>
      <div>
        <label>Class & Level: </label>
        <span>{tomlData.info.classlevel}</span>
      </div>
      {/* Add more fields as needed */}
    </div>
  );
};

export default CharacterSheetPage;
