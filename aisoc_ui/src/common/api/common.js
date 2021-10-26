// Must end in a '/'
export const BASE_URL = '/api/';

class DoxaError extends Error {
  constructor(error_code, error_message) {
    super(`DOXA ERROR: code=${error_code}, message=${error_message}`);
    this.error_code = error_code;
    this.error_message = error_message;
  }
}

async function requestGet({ url, params = {}, authToken = null }) {
  let searchParams = new URLSearchParams();

  for (let key of Object.keys(params)) {
    searchParams.append(key, params[key]);
  }

  const searchParamsString = searchParams.toString();

  return await fetch(url + (searchParamsString ? ('?' + searchParamsString) : ''), {
    method: 'GET',
    headers: authToken ? {
      'Authorization': 'Bearer ' + authToken
    } : undefined,
  });
}

async function requestPost({ url, params = {}, authToken = null }) {
  return await fetch(url, {
    method: 'POST',
    body: JSON.stringify(params),
    headers: authToken ? {
      'Authorization': 'Bearer ' + authToken
    } : undefined,
  });
}


export async function request({ url, params = {}, authToken = null, method = 'GET' }) {
  let response;
  if (method === 'GET') {
    response = await requestGet({ url, params, authToken });
  } else if (method === 'POST') {
    response = await requestPost({ url, params, authToken });
  } else {
    throw new Error(`Unknown method: ${method}`);
  }

  const json = await response.json();

  if (response.status !== 200) {
    throw new DoxaError(json.error_code, json.error_message);
  }

  return json;
}
