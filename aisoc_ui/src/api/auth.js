import { request } from './common';

export async function login(username, password) {
  const response = request({
    url: '/api/auth/login',
    params: { username, password },
    method: 'POST'
  });

  return response.auth_token;
}
