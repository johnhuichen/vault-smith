"use client";

import { useState } from "react";

import Landing from "./Landing";

// Import your vault view component when ready
// import VaultView from './components/VaultView';

function App() {
  const [selectedVaultId, setSelectedVaultId] = useState<string | null>(null);

  const handleVaultSelect = (vaultId: string) => {
    setSelectedVaultId(vaultId);
  };

  // if (selectedVaultId) {
  //   return <VaultView vaultId={selectedVaultId} onBack={() => setSelectedVaultId(null)} />;
  // }

  return <Landing onVaultSelect={handleVaultSelect} />;
}

export default App;
