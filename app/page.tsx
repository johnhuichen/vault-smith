"use client";

import { useState } from "react";

import Landing from "./Landing";
import VaultView from "./VaultView";

function App() {
  const [selectedVaultName, setSelectedVaultName] = useState<string | null>(
    null,
  );

  if (selectedVaultName) {
    return (
      <VaultView
        vaultName={selectedVaultName}
        onBack={() => setSelectedVaultName(null)}
      />
    );
  }

  return <Landing onVaultSelect={setSelectedVaultName} />;
}

export default App;
