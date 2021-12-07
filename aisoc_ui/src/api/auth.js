import { request } from './common';

export async function login(username, password) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/login`,
    params: { username, password },
    method: 'POST'
  });

  return response.auth_token;
}
