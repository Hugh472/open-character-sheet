import React, { useState, useEffect } from 'react';

const WelcomePage: React.FC<{ tomlFiles: string[] }> = ({ tomlFiles }) => {
  return (
    <div className="welcome-page">
      <h1>Welcome to OpenCharacterSheet</h1>
      <h2>Select a Character Sheet:</h2>
      <ul>
        {tomlFiles.map((file, index) => (
          <li key={index}>
            <a href={`/sheet/${file}`}>{file}</a>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default WelcomePage;
