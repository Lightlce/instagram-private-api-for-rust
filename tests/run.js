const fs = require('fs');
const path = require('path');
const assert = require('assert');

function collectTypeScriptEndpoints(root) {
  const urlPattern = /url:\s*([`'\"])(.+?)\1/g;
  const endpointSet = new Set();

  const scanDir = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        scanDir(fullPath);
      } else if (entry.isFile() && fullPath.endsWith('.ts')) {
        const source = fs.readFileSync(fullPath, 'utf8');
        let match;
        while ((match = urlPattern.exec(source)) !== null) {
          endpointSet.add(match[2]);
        }
      }
    }
  };

  scanDir(path.join(root, 'src', 'repositories'));
  scanDir(path.join(root, 'src', 'feeds'));
  scanDir(path.join(root, 'src', 'services'));

  return [...endpointSet].sort();
}

describe('parity matrix smoke checks', () => {
  it('references existing TypeScript and Rust paths', () => {
    const root = path.resolve(__dirname, '..');
    const matrixPath = path.join(root, 'rust', 'parity', 'matrix.json');
    const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));

    assert.ok(Array.isArray(matrix.tracked_groups));
    assert.ok(matrix.tracked_groups.length > 0);

    for (const group of matrix.tracked_groups) {
      assert.ok(group.name && group.name.trim().length > 0);
      for (const relPath of [...group.typescript, ...group.rust]) {
        const fullPath = path.join(root, relPath);
        assert.ok(fs.existsSync(fullPath), `missing mapped path: ${relPath}`);
      }
    }
  });

  it('keeps TypeScript endpoint inventory in parity artifacts', () => {
    const root = path.resolve(__dirname, '..');
    const inventoryPath = path.join(root, 'rust', 'parity', 'typescript-endpoints.txt');
    const inventory = fs
      .readFileSync(inventoryPath, 'utf8')
      .split('\n')
      .map((line) => line.trim())
      .filter((line) => line.length > 0 && !line.startsWith('#'))
      .sort();

    const extracted = collectTypeScriptEndpoints(root);
    assert.deepStrictEqual(
      extracted,
      inventory,
      'typescript endpoint inventory is out of date; refresh rust/parity/typescript-endpoints.txt',
    );
  });
});
