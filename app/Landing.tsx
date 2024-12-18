import { useEffect, useState } from "react";

import {
  faDatabase,
  faExclamationTriangle,
  faGear,
  faKey,
  faPencilAlt,
  faPlus,
  faTimes,
  faTrash,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { invoke } from "@tauri-apps/api/core";
import cx from "classnames";

import Loading from "@/components/widgets/Loading";
import PasswordInput from "@/components/widgets/PasswordInput";

interface Vault {
  name: string;
  metadata: VaultMetaData;
}

interface VaultMetaData {
  created_at: string;
  last_accessed: string;
}

interface LandingProps {
  onVaultSelect: (vaultName: string) => void;
}

function Landing({ onVaultSelect }: LandingProps) {
  const [vaults, setVaults] = useState<Vault[]>([]);
  const [getVaultsLoading, setGetVaultsLoading] = useState(true);

  const [createModalOpen, setCreateModalOpen] = useState(false);
  const [createVaultName, setCreateVaultName] = useState("");
  const [createMasterkey, setCreateMasterkey] = useState("");
  const [createConfirmMasterkey, setCreateConfirmMasterkey] = useState("");
  const [createVaultLoading, setCreateVaultLoading] = useState(false);

  const [updateModalOpen, setUpdateModalOpen] = useState(false);
  const [updateVaultName, setUpdateVaultName] = useState("");
  const [updateOldMasterkey, setUpdateOldMasterkey] = useState("");
  const [updateNewMasterkey, setUpdateNewMasterkey] = useState("");
  const [updateConfirmNewMasterkey, setUpdateConfirmNewMasterkey] =
    useState("");
  const [updateVaultLoading, setUpdateVaultLoading] = useState(false);

  const [renameModalOpen, setRenameModalOpen] = useState(false);
  const [renameOldName, setRenameOldName] = useState("");
  const [renameNewName, setRenameNewName] = useState("");

  const [deleteVaultName, setDeleteVaultName] = useState("");

  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getVaults();
  }, []);

  const getVaults = async () => {
    try {
      const newVaults = await invoke<Vault[]>("list_vaults");
      setVaults(newVaults);
    } catch (error) {
      console.error("Failed to load vaults:", error);
    } finally {
      setGetVaultsLoading(false);
    }
  };

  const handleCreateVault = async () => {
    try {
      setCreateVaultLoading(true);
      const newVault = await invoke<Vault>("create_vault", {
        name: createVaultName,
        masterkey: createMasterkey,
        confirmMasterkey: createConfirmMasterkey,
      });
      setVaults([...vaults, newVault]);
      setCreateVaultName("");
      setCreateMasterkey("");
      setCreateConfirmMasterkey("");
      setCreateModalOpen(false);
    } catch (err) {
      setError(err as string);
    } finally {
      setCreateVaultLoading(false);
    }
  };

  const handleDeleteVault = async (vaultName: string) => {
    try {
      await invoke("delete_vault", { name: vaultName });
      setVaults(vaults.filter((vault) => vault.name !== vaultName));
      setDeleteVaultName("");
    } catch (error) {
      console.error("Failed to delete vault:", error);
    }
  };

  const handleUpdateVault = async () => {
    try {
      setUpdateVaultLoading(true);
      await invoke<Vault>("update_vault", {
        name: updateVaultName,
        oldMasterkey: updateOldMasterkey,
        newMasterkey: updateNewMasterkey,
        confirmNewMasterkey: updateConfirmNewMasterkey,
      });

      setUpdateModalOpen(false);
    } catch (err) {
      setError(err as string);
    } finally {
      setUpdateVaultLoading(false);
    }
  };

  const handleRenameVault = async () => {
    try {
      const updatedVault = await invoke<Vault>("rename_vault", {
        name: renameOldName,
        newName: renameNewName,
      });

      setVaults(
        vaults.map((vault) =>
          vault.name === renameOldName ? updatedVault : vault,
        ),
      );

      setRenameModalOpen(false);
    } catch (err) {
      setError(err as string);
    }
  };

  const openCreateModal = () => {
    setCreateModalOpen(true);
    setError("");
  };

  const openUpdateModal = (vault: Vault) => {
    setError("");
    setUpdateVaultName(vault.name);
    setUpdateOldMasterkey("");
    setUpdateNewMasterkey("");
    setUpdateConfirmNewMasterkey("");
    setUpdateModalOpen(true);
  };

  const openRenameModal = (vault: Vault) => {
    setError("");
    setRenameOldName(vault.name);
    setRenameNewName(vault.name);
    setRenameModalOpen(true);
  };

  const openDeleteModal = (vault: Vault) => {
    setError("");
    setDeleteVaultName(vault.name);
  };

  return (
    <div className="min-h-screen bg-white p-6">
      {/* Header */}
      <div className="max-w-6xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-3xl font-bold text-gray-800 flex items-center gap-3">
            <FontAwesomeIcon icon={faGear} className="" />
            Vault Smith
          </h1>
          <button
            onClick={openCreateModal}
            className="bg-sky-500 hover:bg-sky-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 transition-colors duration-200"
          >
            <FontAwesomeIcon icon={faPlus} />
            New Vault
          </button>
        </div>

        {getVaultsLoading && <Loading className="mx-auto my-4" />}

        {/* Vaults Grid */}
        {!getVaultsLoading && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {vaults.map((vault) => (
              <div
                key={vault.name}
                className="bg-white border border-gray-200 rounded-xl p-6 hover:shadow-lg transition-shadow duration-200"
              >
                {/* Update the vault card actions */}
                <div className="flex justify-between items-start mb-4">
                  <div className="flex items-center gap-3">
                    <FontAwesomeIcon
                      icon={faDatabase}
                      className="text-gray-800 text-xl"
                    />
                    <h2 className="text-xl font-semibold text-gray-800">
                      {vault.name}
                    </h2>
                    <button
                      onClick={() => openRenameModal(vault)}
                      className="text-gray-400 hover:text-sky-500 transition-colors duration-200"
                    >
                      <FontAwesomeIcon icon={faPencilAlt} />
                    </button>
                  </div>
                  <div className="flex gap-3">
                    <button
                      onClick={() => openUpdateModal(vault)}
                      className="text-gray-400 hover:text-sky-500 transition-colors duration-200"
                    >
                      <FontAwesomeIcon icon={faKey} />
                    </button>
                    <button
                      onClick={() => openDeleteModal(vault)}
                      className="text-gray-400 hover:text-red-500 transition-colors duration-200"
                    >
                      <FontAwesomeIcon icon={faTrash} />
                    </button>
                  </div>
                </div>

                <p className="text-sm text-gray-500 mb-1">
                  Created:{" "}
                  {new Date(vault.metadata.created_at).toLocaleString()}
                </p>
                <p className="text-sm text-gray-500 mb-4">
                  Last accessed:{" "}
                  {new Date(vault.metadata.last_accessed).toLocaleString()}
                </p>
                <button
                  onClick={() => onVaultSelect(vault.name)}
                  className="w-full bg-sky-50 hover:bg-sky-100 text-sky-600 py-2 rounded-lg transition-colors duration-200"
                >
                  Open Vault
                </button>
              </div>
            ))}
          </div>
        )}

        {/* Rename Vault Modal */}
        {renameModalOpen && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Rename Vault
                </h2>
                <button
                  onClick={() => {
                    setRenameModalOpen(false);
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
                placeholder="Enter a new vault name"
                value={renameNewName}
                onChange={(e) => setRenameNewName(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent mb-4"
              />
              <div className="flex justify-end gap-3">
                <button
                  onClick={() => {
                    setRenameModalOpen(false);
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

        {/* Update Vault Modal */}
        {updateModalOpen && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Update master key
                </h2>
                <button
                  onClick={() => {
                    setUpdateModalOpen(false);
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

              <h2 className="font-bold text-gray-800 mb-1">
                Current master key
              </h2>
              <PasswordInput
                placeholder="Enter the current master key"
                value={updateOldMasterkey}
                onChange={setUpdateOldMasterkey}
                className="mb-4"
              />
              <h2 className="font-bold text-gray-800 mb-1">New master key</h2>
              <PasswordInput
                placeholder="Enter a new master key"
                value={updateNewMasterkey}
                onChange={setUpdateNewMasterkey}
                className="mb-4"
              />
              <h2 className="font-bold text-gray-800 mb-1">
                Confirm New master key
              </h2>
              <PasswordInput
                placeholder="Enter a new master key"
                value={updateConfirmNewMasterkey}
                onChange={setUpdateConfirmNewMasterkey}
                className="mb-4"
              />
              <div className="flex justify-end gap-3">
                <button
                  onClick={() => {
                    setUpdateModalOpen(false);
                  }}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleUpdateVault}
                  className={cx(
                    "flex justify-center items-center w-[120px]",
                    "px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200",
                  )}
                >
                  {updateVaultLoading && <Loading className="h-5 scale-50" />}
                  {!updateVaultLoading && "Update"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Create Vault Modal */}
        {createModalOpen && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Create New Vault
                </h2>
                <button
                  onClick={() => {
                    setCreateModalOpen(false);
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

              <h2 className="font-bold text-gray-800 mb-1">Vault name</h2>
              <input
                type="text"
                placeholder="Enter a name"
                value={createVaultName}
                onChange={(e) => setCreateVaultName(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent mb-4"
              />
              <h2 className="font-bold text-gray-800 mb-1">Master key</h2>
              <PasswordInput
                value={createMasterkey}
                onChange={setCreateMasterkey}
                placeholder="Enter a master key"
                className="mb-4"
              />
              <h2 className="font-bold text-gray-800 mb-1">
                Confirm Master key
              </h2>
              <PasswordInput
                value={createConfirmMasterkey}
                onChange={setCreateConfirmMasterkey}
                placeholder="Enter a master key"
                className="mb-4"
              />
              <div className="flex justify-end gap-3">
                <button
                  onClick={() => {
                    setCreateModalOpen(false);
                  }}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleCreateVault}
                  className={cx(
                    "flex justify-center items-center w-[120px]",
                    "px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200",
                  )}
                >
                  {createVaultLoading && <Loading className="h-5 scale-50" />}
                  {!createVaultLoading && "Create Vault"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Delete Confirmation Modal */}
        {deleteVaultName && (
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
                  onClick={() => setDeleteVaultName("")}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={() => handleDeleteVault(deleteVaultName)}
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
