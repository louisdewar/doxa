import { BASE_URL, request } from './common';

const LEADERBOARD_BASE_URL = BASE_URL + 'competition/uttt/_leaderboard/';

export async function getActive() {
  const data = await request({ url: LEADERBOARD_BASE_URL + 'active', method: 'GET' });

  return data.leaderboard;
}

export default { getActive };