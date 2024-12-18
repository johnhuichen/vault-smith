import { useState } from "react";

import {
  faCheck,
  faCopy,
  faEye,
  faEyeSlash,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

interface PasswordInputProps {
  value: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  label?: string;
  error?: string;
  disabled?: boolean;
  className?: string;
}

function PasswordInput({
  value,
  onChange = () => {},
  placeholder = "Enter password",
  label = "",
  error = "",
  disabled = false,
  className = "",
}: PasswordInputProps) {
  const [showPassword, setShowPassword] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(value);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000); // Reset after 2 seconds
  };

  return (
    <div className={className}>
      {label && (
        <label className="block text-sm font-medium text-gray-700 mb-1">
          {label}
        </label>
      )}
      <div className="relative">
        <input
          type={showPassword ? "text" : "password"}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
          className={`
            w-full px-4 py-2 pr-24  // Increased right padding for both icons
            rounded-lg border 
            focus:outline-none focus:ring-2 focus:ring-sky-500 focus:border-transparent
            ${error ? "border-red-500" : "border-gray-300"}
          `}
        />
        <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2">
          {/* Copy button */}
          <button
            type="button"
            onClick={handleCopy}
            className="text-gray-500 hover:text-gray-700 focus:outline-none transition-colors duration-200"
            title={copied ? "Copied!" : "Copy to clipboard"}
          >
            <FontAwesomeIcon
              icon={copied ? faCheck : faCopy}
              className={copied ? "text-sky-500" : ""}
            />
          </button>
          {/* Show/Hide password button */}
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="text-gray-500 hover:text-gray-700 focus:outline-none transition-colors duration-200"
            title={showPassword ? "Hide password" : "Show password"}
          >
            <FontAwesomeIcon icon={showPassword ? faEye : faEyeSlash} />
          </button>
        </div>
      </div>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
}

export default PasswordInput;
