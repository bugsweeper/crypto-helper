import * as wasm from "algorithms-wasm";

const CURRENT_ALGORITHM_INFO = 'current-algorithm-info';
const CURRENT_ALGORITHM_IN = 'current-algorithm-in';
const CURRENT_ALGORITHM_OUT = 'current-algorithm-out';

try {
  console.log(wasm.simple_hash('crypto-helper'));
} catch(e) {
  console.dir({ e });
}

const getAlgorithmFn = algorithm => {
  if (algorithm.startsWith('sha')) {
    return algorithm;
  } else if (algorithm === 'aes256-cts-hmac-sha1-96') {
    return 'aes256_cts_hmac_sha1_96_encrypt';
  }
};

const getAlgorithm = () => document.getElementById('algorithm').selectedOptions[0].value;

const collectData = algorithm => {
  if (algorithm.startsWith('sha')) {
    return [document.getElementById(`${algorithm}-indata`).value];
  } else if (algorithm === 'aes256-cts-hmac-sha1-96') {
    const key = document.getElementById(`${algorithm}-key-input`).value;
    const usage = +document.getElementById(`${algorithm}-inusage`).value;
    const payload = document.getElementById(`${algorithm}-payload-input`).value;

    return [key, usage, payload];
  } else {
    showNotification({ type: 'error', text: `${algorithm} not implemented yet` })
    throw new Error('not implemented yet');
  }
};

const setData = (algorithm, data) => {
  if (algorithm.startsWith('sha')) {
    document.getElementById(`${algorithm}-outdata`).innerText = data;
  } else if (algorithm === 'aes256-cts-hmac-sha1-96') {
    const len = data.length;
    document.getElementById(`${algorithm}-cipher`).innerText = data.substring(0, len - 24);
    document.getElementById(`${algorithm}-hmac`).innerText = data.substring(len - 24);
    document.getElementById(`${algorithm}-total-len`).innerText = len / 2;
    document.getElementById(`${algorithm}-cipher-len`).innerText = (len / 2) - 12;
  } else {
    showNotification({ type: 'error', text: `${algorithm} not implemented yet` })
    throw new Error('not implemented yet');
  }
};

const go = () => {
  const algorithm = getAlgorithm();
  const dataIn = collectData(algorithm);
  try {
    const dataOut = wasm[getAlgorithmFn(algorithm)](...dataIn);
    setData(algorithm, dataOut);
  } catch(e) {
    showNotification({ type: 'error', text: e });
  }
};

const toggleAutoConvert = () => {
  if (document.getElementById('autoConvert').checked) {
    for (const algorithm of ['sha1', 'sha256', 'sha512']) {
      const indata = document.getElementById(`${algorithm}-indata`);
      indata.addEventListener('change', go);
      indata.addEventListener('input', go);
    }
  } else {
    for (const algorithm of ['sha1', 'sha256', 'sha512']) {
      const indata = document.getElementById(`${algorithm}-indata`);
      indata.removeEventListener('change', go);
      indata.removeEventListener('input', go);
    }
  }
};

document
  .getElementById('go')
  .addEventListener('click', go);

document
  .getElementById('autoConvert')
  .addEventListener('change', toggleAutoConvert);

document.addEventListener('keypress', event => {
    if (event.ctrlKey && event.code === 'Enter') {
        go();
    }
});

toggleAutoConvert();
// go();
