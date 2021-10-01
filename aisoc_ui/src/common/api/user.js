import { BASE_URL, request } from './common';

const USER_BASE_URL = BASE_URL + 'competition/uttt/_user/';

export async function getActiveAgent(username) {
  const data = await request({ url: USER_BASE_URL + username + '/active_agent', method: 'GET' });

  return data.active_agent;
}

export async function getScore(username) {
  const data = await request({ url: USER_BASE_URL + username + '/score', method: 'GET' });

  return data;
}

export async function getActiveGames(username) {
  const data = await request({ url: USER_BASE_URL + username + '/active_games', method: 'GET' });

  return data.games;
}

export default { getScore, getActiveAgent, getActiveGames };
