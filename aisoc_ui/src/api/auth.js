import { request } from './common';

export async function getUserInfo(authToken) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}user/info`,
    authToken,
    method: 'POST'
  });

  return response;
}

export async function login(username, password) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/login`,
    params: { username, password },
    method: 'POST'
  });

  return response.auth_token;
}

export async function getInviteInfo(id) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/invite/info/${id}`,
    method: 'GET'
  });

  return response;
}

export async function acceptInvite(id, username, password) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/invite/accept/${id}`,
    method: 'POST',
    params: { username, password }
  });

  return response;
}
