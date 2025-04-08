import { useEffect, useState } from 'react';

const ImageConverter = () => {
  const [wasm, setWasm] = useState<any>(null);
  const [result, setResult] = useState<Uint8Array | null>(null);

  useEffect(() => {
    import('/pkg/img2webp').then(module => {
      setWasm(module);
    });
  }, []);

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file || !wasm) return;

    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    const converted = wasm.process_image_to_webp(uint8Array);
    setResult(converted);
  };

  return (
    <div>
      <input type="file" accept=".png,.jpg,.jpeg,.bmp,.webp" onChange={handleFileUpload} />
      {result && <p>Converted to WebP: {result.length} bytes</p>}
    </div>
  );
};

export default ImageConverter;