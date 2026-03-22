const fs = require('fs');
const path = require('path');
const assert = require('assert');

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
});
