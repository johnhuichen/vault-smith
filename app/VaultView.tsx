import { useState } from "react";

import {
  faCircleLeft,
  faCopy,
  faEdit,
  faPlus,
  faSearch,
  faStickyNote,
  faTimes,
  faTrash,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { invoke } from "@tauri-apps/api/core";
import cx from "classnames";

import Loading from "@/components/widgets/Loading";
import PasswordInput from "@/components/widgets/PasswordInput";

interface VaultViewProps {
  vaultName: string;
  onBack: () => void;
}

interface Password {
  id: string;
  password: string;
  notes: string;
}

export default function VaultView({ vaultName, onBack }: VaultViewProps) {
  const [passwords, setPasswords] = useState<Password[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [masterKey, setMasterKey] = useState("");
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [error, setError] = useState<string>("");

  // Modal states
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [newNotes, setNewNotes] = useState("");
  const [editingPassword, setEditingPassword] = useState<Password | null>(null);
  const [passwordToDelete, setPasswordToDelete] = useState<Password | null>(
    null,
  );

  // Loading states
  const [unlockLoading, setUnlockLoading] = useState(false);
  const [addPasswordLoading, setAddPasswordLoading] = useState(false);
  const [updatePasswordLoading, setUpdatePasswordLoading] = useState(false);
  const [deletePasswordLoading, setDeletePasswordLoading] = useState(false);

  const handleUnlock = async () => {
    try {
      setUnlockLoading(true);
      setError("");
      const result = await invoke<Password[]>("get_passwords", {
        vaultName,
        masterkey: masterKey,
      });
      setPasswords(result);
      setIsUnlocked(true);
    } catch (err) {
      setError(err as string);
    } finally {
      setUnlockLoading(false);
    }
  };

  const handleAddPassword = async () => {
    try {
      setAddPasswordLoading(true);
      setError("");
      await invoke("add_password", {
        vaultName,
        masterkey: masterKey,
        notes: newNotes,
      });

      // Refresh passwords list
      const updated = await invoke<Password[]>("get_passwords", {
        vaultName,
        masterkey: masterKey,
      });
      setPasswords(updated);
      setIsAddModalOpen(false);
      setNewNotes("");
    } catch (err) {
      setError(err as string);
    } finally {
      setAddPasswordLoading(false);
    }
  };

  const handleDeletePassword = async (password: Password) => {
    try {
      setDeletePasswordLoading(true);
      await invoke("delete_password", {
        vaultName,
        masterkey: masterKey,
        id: password.id,
      });

      // Refresh passwords list
      const updated = await invoke<Password[]>("get_passwords", {
        vaultName,
        masterkey: masterKey,
      });
      setPasswords(updated);
      setPasswordToDelete(null);
    } catch (err) {
      setError(err as string);
    } finally {
      setDeletePasswordLoading(false);
    }
  };

  const handleUpdatePassword = async (
    id: string,
    updatedPassword: string,
    updatedNotes: string,
  ) => {
    try {
      setUpdatePasswordLoading(true);
      await invoke("update_password", {
        vaultName,
        masterkey: masterKey,
        id: id,
        password: updatedPassword,
        notes: updatedNotes,
      });

      // Refresh passwords list
      const updated = await invoke<Password[]>("get_passwords", {
        vaultName,
        masterkey: masterKey,
      });
      setPasswords(updated);
      setEditingPassword(null);
    } catch (err) {
      setError(err as string);
    } finally {
      setUpdatePasswordLoading(false);
    }
  };

  const filteredPasswords = passwords.filter((entry) =>
    entry.notes.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  if (!isUnlocked) {
    return (
      <div className="min-h-screen bg-white p-6">
        <div className="max-w-md mx-auto mt-20">
          <div className="flex items-center gap-4 mb-6">
            <button
              onClick={onBack}
              className="text-2xl text-gray-600 hover:text-gray-800 transition-colors duration-200"
            >
              <FontAwesomeIcon icon={faCircleLeft} />
            </button>
            <h2 className="text-2xl font-bold text-gray-800">Unlock Vault</h2>
          </div>
          {error && (
            <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-lg text-sm">
              {error}
            </div>
          )}
          <PasswordInput
            value={masterKey}
            onChange={setMasterKey}
            placeholder="Enter master key"
            className="mb-4"
          />
          <button
            onClick={handleUnlock}
            className={cx(
              "w-full bg-sky-500 hover:bg-sky-600 text-white px-4 py-2 rounded-lg transition-colors duration-200",
              "flex justify-center items-center",
            )}
          >
            {unlockLoading && <Loading className="h-5 scale-50" />}
            {!unlockLoading && "Unlock"}
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-white p-6">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <div className="flex justify-between items-center mb-8">
          <div className="flex items-center gap-4">
            <button
              onClick={onBack}
              className="text-3xl text-gray-600 hover:text-gray-800 transition-colors duration-200"
            >
              <FontAwesomeIcon icon={faCircleLeft} />
            </button>
            <h1 className="text-3xl font-bold text-gray-800">{vaultName}</h1>
          </div>
          <button
            onClick={() => setIsAddModalOpen(true)}
            className={cx(
              "bg-sky-500 hover:bg-sky-600 text-white px-4 py-2 rounded-lg flex items-center gap-2 transition-colors duration-200",
            )}
          >
            <FontAwesomeIcon icon={faPlus} />
            Add Password
          </button>
        </div>

        {/* Search */}
        <div className="mb-6">
          <div className="relative">
            <FontAwesomeIcon
              icon={faSearch}
              className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400"
            />
            <input
              type="text"
              placeholder="Search notes..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent"
            />
          </div>
        </div>

        {/* Passwords List */}
        <div className="grid gap-4">
          {filteredPasswords.map((entry) => (
            <div
              key={entry.id}
              className="bg-white border border-gray-200 rounded-xl p-4 hover:shadow-lg transition-shadow duration-200"
            >
              <div className="flex justify-between items-start">
                <div className="flex-1">
                  <div className="flex items-center gap-2 mb-2">
                    <FontAwesomeIcon
                      icon={faStickyNote}
                      className="text-gray-400"
                    />
                    <div className="text-gray-600 whitespace-pre-wrap">
                      {entry.notes}
                    </div>
                  </div>
                  <PasswordInput
                    value={entry.password}
                    disabled
                    className="bg-gray-50"
                  />
                </div>
                <div className="flex gap-3 ml-4">
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(entry.password);
                      // TODO: Add toast notification
                    }}
                    className="text-gray-400 hover:text-sky-500 transition-colors duration-200"
                  >
                    <FontAwesomeIcon icon={faCopy} />
                  </button>
                  <button
                    onClick={() => setEditingPassword(entry)}
                    className="text-gray-400 hover:text-sky-500 transition-colors duration-200"
                  >
                    <FontAwesomeIcon icon={faEdit} />
                  </button>
                  <button
                    onClick={() => setPasswordToDelete(entry)}
                    className="text-gray-400 hover:text-red-500 transition-colors duration-200"
                  >
                    <FontAwesomeIcon icon={faTrash} />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Add Password Modal */}
        {isAddModalOpen && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Add Password
                </h2>
                <button
                  onClick={() => setIsAddModalOpen(false)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <FontAwesomeIcon icon={faTimes} />
                </button>
              </div>

              {error && (
                <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-lg text-sm">
                  {error}
                </div>
              )}

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Notes
                </label>
                <textarea
                  value={newNotes}
                  onChange={(e) => setNewNotes(e.target.value)}
                  placeholder="Add notes about this password..."
                  className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent"
                  rows={3}
                />
              </div>

              <div className="flex justify-end gap-3">
                <button
                  onClick={() => setIsAddModalOpen(false)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={handleAddPassword}
                  className={cx(
                    "flex justify-center items-center w-[150px]",
                    "px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200",
                  )}
                >
                  {addPasswordLoading && <Loading className="h-5 scale-50" />}
                  {!addPasswordLoading && "Add Password"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Edit Password Modal */}
        {editingPassword && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-2xl font-bold text-gray-800">
                  Edit Password
                </h2>
                <button
                  onClick={() => setEditingPassword(null)}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <FontAwesomeIcon icon={faTimes} />
                </button>
              </div>

              {error && (
                <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-lg text-sm">
                  {error}
                </div>
              )}

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Notes
                </label>
                <textarea
                  value={editingPassword.notes}
                  onChange={(e) =>
                    setEditingPassword({
                      ...editingPassword,
                      notes: e.target.value,
                    })
                  }
                  placeholder="Add notes about this password..."
                  className="w-full px-4 py-2 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent"
                  rows={3}
                />
              </div>

              <div className="mb-4">
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Password
                </label>
                <PasswordInput
                  value={editingPassword.password}
                  onChange={(value) =>
                    setEditingPassword({
                      ...editingPassword,
                      password: value,
                    })
                  }
                  placeholder="Enter password"
                />
              </div>

              <div className="flex justify-end gap-3">
                <button
                  onClick={() => setEditingPassword(null)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={() =>
                    handleUpdatePassword(
                      editingPassword.id,
                      editingPassword.password,
                      editingPassword.notes,
                    )
                  }
                  className={cx(
                    "flex justify-center items-center w-[150px]",
                    "px-4 py-2 bg-sky-500 hover:bg-sky-600 text-white rounded-lg transition-colors duration-200",
                  )}
                >
                  {updatePasswordLoading && (
                    <Loading className="h-5 scale-50" />
                  )}
                  {!updatePasswordLoading && "Save Changes"}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Delete Confirmation Modal */}
        {passwordToDelete && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-xl p-6 w-full max-w-md">
              <div className="flex items-center gap-3 mb-4">
                <FontAwesomeIcon
                  icon={faTrash}
                  className="text-red-500 text-xl"
                />
                <h2 className="text-2xl font-bold text-gray-800">
                  Delete Password
                </h2>
              </div>

              <p className="text-gray-600 mb-2">
                Are you sure you want to delete this password?
              </p>
              <p className="text-gray-600 mb-6">
                <strong className="text-red-600">
                  This action cannot be undone.
                </strong>
              </p>

              <div className="bg-gray-50 rounded-lg p-4 mb-6">
                <div className="text-sm text-gray-500 mb-1">Notes</div>
                <div className="text-gray-700">{passwordToDelete.notes}</div>
              </div>

              <div className="flex justify-end gap-3">
                <button
                  onClick={() => setPasswordToDelete(null)}
                  className="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors duration-200"
                >
                  Cancel
                </button>
                <button
                  onClick={() => handleDeletePassword(passwordToDelete)}
                  className={cx(
                    "flex justify-center items-center w-[160px]",
                    "px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded-lg transition-colors duration-200",
                  )}
                >
                  {deletePasswordLoading && (
                    <Loading className="h-5 scale-50" />
                  )}
                  {!deletePasswordLoading && "Delete Password"}
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
