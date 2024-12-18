export const glossary: Record<string, string> = {
  "master key": `A crucial security measure: your master key is never stored in the application. Please ensure you store it in a secure location. If you lost a master key, your vault cannot be accessed and the lost key cannot be recovered.`,
  password: `For enhanced security, the application automatically generates strong, random passwords. You can modify these passwords at any time while maintaining them in your vault.`,
  vault: `Your vaults are stored as encrypted files on your local device, using master keys. Regular backups of these vault files are strongly recommended, as lost or corrupted files cannot be recovered. Without access to these files, all stored passwords will be permanently lost.`,
  notes:
    "A useful field for storing password-related information. For enhanced security, consider using hints rather than explicit details. For example: 'Work email / jd initials' is safer than 'john.doe@company.com'. This way, you can identify your passwords while keeping sensitive information protected.",
};
