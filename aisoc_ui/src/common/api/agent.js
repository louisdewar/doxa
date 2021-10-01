import { BASE_URL, request } from './common';

const AGENT_BASE_URL = BASE_URL + 'competition/uttt/_agent/';

export async function getGames(agent) {
  const data = await request({ url: AGENT_BASE_URL + agent + '/games', method: 'GET' });

  return data.games;
}

export async function getScore(agent) {
  const data = await request({ url: AGENT_BASE_URL + agent + '/score', method: 'GET' });

  return data;
}

export default { getGames, getScore };