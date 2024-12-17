import { useEffect, useState } from "react";

import {
  faExclamationTriangle,
  faLock,
  faPencilAlt,
  faPlus,
  faTimes,
  faTrash,
  faUserLock,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { invoke } from "@tauri-apps/api/core";

import Loading from "@/components/widgets/Loading";

interface Vault {
  id: string;
  name: string;
  lastAccessed: string;
}

interface LandingProps {
  onVaultSelect: (vaultId: string) => void;
}

function Landing({ onVaultSelect }: LandingProps) {
  const [vaults, setVaults] = useState<Vault[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [newVaultName, setNewVaultName] = useState("");
  const [newMasterkey, setNewMasterkey] = useState("");
  const [deleteVaultId, setDeleteVaultId] = useState<string | null>(null);
  const [isRenameModalOpen, setIsRenameModalOpen] = useState(false);
  const [renameVaultId, setRenameVaultId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadVaults();
  }, []);

  const loadVaults = async () => {
    try {
      // const vaultList = await invoke<Vault[]>("list_vaults");
      // setVaults(vaultList);
      setVaults([]);
    } catch (error) {
      console.error("Failed to load vaults:", error);
      // Handle error (show notification, etc.)
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateVault = async () => {
    try {
      setError(null);
      const newVault = await invoke<Vault>("create_vault", {
        name: newVaultName.trim(),
        masterkey: newMasterkey,
      });
      // setVaults([...vaults, newVault]);
      setNewVaultName("");
      setNewMasterkey("");
      setIsCreateModalOpen(false);
    } catch (err) {
      setError(err as string);
      // Show error message to user
    }
  };

  const handleDeleteVault = async (vaultId: string) => {
    try {
      await invoke("delete_vault", { name: vaultId });
      setVaults(vaults.filter((vault) => vault.id !== vaultId));
      setDeleteVaultId(null);
    } catch (error) {
      console.error("Failed to delete vault:", error);
      // Show error message to user
    }
  };

  const handleRenameVault = async () => {
    if (!renameVaultId || !newVaultName.trim()) return;

    try {
      const updatedVault = await invoke<Vault>("rename_vault", {
        oldName: renameVaultId,
        newName: newVaultName.trim(),
      });

      setVaults(
        vaults.map((vault) =>
          vault.id === renameVaultId ? updatedVault : vault,
        ),
      );

      setRenameVaultId(null);
      setNewVaultName("");
      setIsRenameModalOpen(false);
    } catch (err) {
      setError(err as string);
    }
  };

  const openRenameModal = (vault: Vault) => {
    setRenameVaultId(vault.id);
    setNewVaultName(vault.name);
    setIsRenameModalOpen(true);
  };

  if (isLoading) {
    return <Loading />;
  }

  return (
    <div className="min-h-screen bg-white p-6">
      {/* Header */}
      <div className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-3xl font-bold text-gray-800 flex items-center gap-3">
            <FontAwesomeIcon icon={faUserLock} className="" />
            Password Vaults
          </h1>
          <button
            onClick={() => setIsCreateModalOpen(true)}
            className="bg-sky-500 hover:bg-sky-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 transition-colors duration-200"
          >
            <FontAwesomeIcon icon={faPlus} />
            New Vault
          </button>
        </div>

        {/* Vaults Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {vaults.map((vault) => (
            <div
              key={vault.id}
              className="bg-white border border-gray-200 rounded-xl p-6 hover:shadow-lg transition-shadow duration-200"
            >
              {/* Update the vault card actions */}
              <div className="flex justify-between items-start mb-4">
                <div className="flex items-center gap-3">
                  <FontAwesomeIcon
                    icon={faLock}
                    className="text-sky-500 text-xl"
                  />
                  <h2 className="text-xl font-semibold text-gray-800">
                    {vault.name}
                  </h2>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => openRenameModal(vault)}
                    className="text-gray-400 hover:text-sky-500 transition-colors duration-200"
                  >
                    <FontAwesomeIcon icon={faPencilAlt} />
                  </button>
                  <button
                    onClick={() => setDeleteVaultId(vault.id)}
                    className="text-gray-400 hover:text-red-500 transition-colors duration-200"
                  >
                    <FontAwesomeIcon icon={faTrash} />
                  </button>
                </div>
              </div>

              {/* Add Rename Modal */}
              {isRenameModalOpen && (
                <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
                  <div className="bg-white rounded-xl p-6 w-full max-w-md">
                    <div className="flex justify-between items-center mb-4">
                      <h2 className="text-2xl font-bold text-gray-800">
                        Rename Vault
                      </h2>
                      <button
                        onClick={() => {
                          setIsRenameModalOpen(false);
                          setRenameVaultId(null);
                          setNewVaultName("");
                        }}
                        className="text-gray-400 hover:text-gray-600"
                      >
                        <FontAwesomeIcon icon={faTimes} />
                      </button>
                    </div>

                    {/* Error Message */}
                    {error && (
                      <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-lg text-sm">
                        {error}
                      </div>
                    )}

                    <input
                      type="text"
                      placeholder="New vault name"
                      value={newVaultName}
                      onChange={(e) => setNewVaultName(e.target.value)}
                      className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent mb-4"
                    />
                    <div className="flex justify-end gap-3">
                      <button
                        onClick={() => {
                          setIsRenameModalOpen(false);
                          setRenameVaultId(null);
                          setNewVaultName("");
                        }}
                        className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                      >
                        Cancel
                      </button>
                      <button
                        onClick={handleRenameVault}
                        className="px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200"
                      >
                        Rename
                      </button>
                    </div>
                  </div>
                </div>
              )}

              <p className="text-sm text-gray-500 mb-4">
                Last accessed: {vault.lastAccessed}
              </p>
              <button
                onClick={() => onVaultSelect(vault.id)}
                className="w-full bg-sky-50 hover:bg-sky-100 text-sky-600 py-2 rounded-lg transition-colors duration-200"
              >
                Open Vault
              </button>
            </div>
          ))}
        </div>

        {/* Create Vault Modal */}
        {isCreateModalOpen && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Create New Vault
                </h2>
                <button
                  onClick={() => setIsCreateModalOpen(false)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <FontAwesomeIcon icon={faTimes} />
                </button>
              </div>

              {/* Error Message */}
              {error && (
                <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-lg text-sm">
                  {error}
                </div>
              )}

              <input
                type="text"
                placeholder="Vault name"
                value={newVaultName}
                onChange={(e) => setNewVaultName(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent mb-4"
              />
              <input
                type="text"
                placeholder="Master key"
                value={newMasterkey}
                onChange={(e) => setNewMasterkey(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent mb-4"
              />
              <div className="flex justify-end gap-3">
                <button
                  onClick={() => setIsCreateModalOpen(false)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleCreateVault}
                  className="px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200"
                >
                  Create Vault
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Delete Confirmation Modal */}
        {deleteVaultId && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex items-center gap-3 mb-4">
                <FontAwesomeIcon
                  icon={faExclamationTriangle}
                  className="text-red-500 text-xl"
                />
                <h2 className="text-2xl font-bold text-gray-800">
                  Delete Vault
                </h2>
              </div>
              <p className="text-gray-600 mb-6">
                Are you sure you want to delete this vault? This action cannot
                be undone.
              </p>
              <div className="flex justify-end gap-3">
                <button
                  onClick={() => setDeleteVaultId(null)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={() => handleDeleteVault(deleteVaultId)}
                  className="px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors duration-200"
                >
                  Delete Vault
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default Landing;
