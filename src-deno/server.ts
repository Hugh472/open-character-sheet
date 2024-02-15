// src-deno/server.ts
import { serve } from 'https://deno.land/std/http/server.ts';
import { readDir } from 'https://deno.land/std/fs/mod.ts';
import { fromFile } from 'https://deno.land/x/parse_toml/mod.ts';

const PORT = 8000;

const server = serve({ port: PORT });
console.log(`Server is running on http://localhost:${PORT}`);

const tomlDir = '.opencharactersheet';

async function listTomlFiles(): Promise<string[]> {
  try {
    const files = await readDir(tomlDir);
    return files.filter((file) => file.name.endsWith('.toml')).map((file) => file.name);
  } catch (error) {
    console.error('Error listing TOML files:', error);
    return [];
  }
}

async function getTomlData(fileName: string): Promise<Record<string, any> | null> {
  try {
    const fileContent = await Deno.readTextFile(`${tomlDir}/${fileName}`);
    return fromFile(fileName, fileContent);
  } catch (error) {
    console.error('Error reading TOML file:', error);
    return null;
  }
}

for await (const req of server) {
  if (req.url === '/api/toml-files') {
    const tomlFiles = await listTomlFiles();
    req.respond({ body: JSON.stringify(tomlFiles) });
  } else if (req.url?.startsWith('/api/toml/')) {
    const fileName = req.url.substring('/api/toml/'.length);
    const tomlData = await getTomlData(fileName);
    req.respond({ body: JSON.stringify(tomlData) });
  } else {
    // Serve the React app
    const htmlContent = await Deno.readTextFile('index.html');
    req.respond({ body: htmlContent, headers: new Headers({ 'Content-Type': 'text/html' }) });
  }
}
