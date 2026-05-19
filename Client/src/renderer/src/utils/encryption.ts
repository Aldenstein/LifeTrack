// utils/encryption.ts

function hexToBytes(hex: string): Uint8Array {
  return new Uint8Array(hex.match(/.{1,2}/g)!.map((b) => parseInt(b, 16)))
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('')
}

async function importKey(hexKey: string, usage: 'encrypt' | 'decrypt'): Promise<CryptoKey> {
  // Fix : .buffer as ArrayBuffer — evite l'erreur Uint8Array<ArrayBufferLike>
  return crypto.subtle.importKey(
    'raw',
    hexToBytes(hexKey).buffer as ArrayBuffer,
    { name: 'AES-CBC' },
    false,
    [usage]
  )
}

export async function encryptData(
  data: unknown,
  hexKey: string
): Promise<{ iv: string; ciphertext: string }> {
  const cryptoKey = await importKey(hexKey, 'encrypt')
  const iv        = crypto.getRandomValues(new Uint8Array(16))
  const encoded   = new TextEncoder().encode(JSON.stringify(data))
  // Fix : cast ArrayBuffer sur iv et encoded
  const encrypted = await crypto.subtle.encrypt(
    { name: 'AES-CBC', iv: iv.buffer as ArrayBuffer },
    cryptoKey,
    encoded.buffer as ArrayBuffer
  )
  return {
    iv:         bytesToHex(iv),
    ciphertext: bytesToHex(new Uint8Array(encrypted)),
  }
}

export async function decryptData<T>(
  iv: string,
  ciphertext: string,
  hexKey: string
): Promise<T> {
  const cryptoKey = await importKey(hexKey, 'decrypt')
  // Fix : cast ArrayBuffer sur iv et ciphertext
  const decrypted = await crypto.subtle.decrypt(
    { name: 'AES-CBC', iv: hexToBytes(iv).buffer as ArrayBuffer },
    cryptoKey,
    hexToBytes(ciphertext).buffer as ArrayBuffer
  )
  return JSON.parse(new TextDecoder().decode(decrypted)) as T
}
