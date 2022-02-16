export class DoxaError extends Error {
  constructor(error_code, error_message, status_code) {
    super(`DOXA ERROR: code=${error_code}, message=${error_message}, status_coed=${status_code}`);
    this.error_code = error_code;
    this.error_message = error_message;
    this.status_code = status_code;
  }
}

export async function requestGet({ url, params = {}, authToken = null }) {
  const searchParams = new URLSearchParams();
  for (const [key, value] of Object.entries(params)) {
    searchParams.append(key, value);
  }

  return await fetch(`${url}?${searchParams}`, {
    method: 'GET',
    headers: authToken ? {
      'Authorization': 'Bearer ' + authToken
    } : undefined,
  });
}

export async function requestPost({ url, params = {}, authToken = null }) {
  const headers = {
    'Content-Type': 'application/json'
  };

  if (authToken) {
    headers['Authorization'] = `Bearer ${authToken}`;
  }

  return await fetch(url, {
    method: 'POST',
    body: JSON.stringify(params),
    headers: headers,
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

  let json;
  try {
    json = await response.json();
  } catch {
    throw new Error('Improperly formatted error');
  }

  if (response.status !== 200) {
    throw new DoxaError(json.error_code, json.error, response.status);
  }

  return json;
}
