import VerifyEmailCard from 'components/VerifyEmailCard';
import { request } from './common';

class ProviderFlowResponse {
  constructor(payload) {
    this.payload = payload;
    this.type = payload.type;
  }

  authToken() {
    if (this.type === 'authenticated') {
      return this.payload.auth_token;
    } else {
      return null;
    }
  }

  getFlowType() {
    return this.type;
  }

  incomplete(flowHandler) {
    if (this.type !== 'incomplete') return null;

    // Get inner incomplete flow payload
    const payload = this.payload.payload;
    const type = payload.type;

    if (type === 'verify_email') {
      return <VerifyEmailCard startLetter={payload.start_letter} domain={payload.domain} />;
    } else if (flowHandler && typeof flowHandler === 'function') {
      return flowHandler(type, payload);
    } else {
      throw new Error(`Unhandled incomplete flow ${type}`);
    }
  }
}

export async function getUserInfo(authToken) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}user/info`,
    authToken,
    method: 'POST'
  });

  return response;
}

export async function login(usernameOrEmail, password) {
  const response = await providerFlow('password', 'login', { username_or_email: usernameOrEmail, password });

  return response;
}

export async function register(username, email, password) {
  const response = await providerFlow('password', 'register', { username, email, password });

  return response;
}

export async function providerFlow(providerName, flowName, payload) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/provider_flow`,
    params: { flow_name: flowName, provider_name: providerName, payload },
    method: 'POST'
  });

  return new ProviderFlowResponse(response);
}
export async function verifyEmail(verificationCode) {
  const response = await request({
    url: `${process.env.REACT_APP_API_BASE_URL}auth/verify_email`,
    params: { verification_code: verificationCode },
    method: 'POST'
  });

  return new ProviderFlowResponse(response);
}

export async function authorizeDelegatedLogin(authToken, verificationCode) {
  await request({
    authToken,
    url: `${process.env.REACT_APP_API_BASE_URL}auth/authorize_delegated`,
    params: { verification_code: verificationCode },
    method: 'POST'
  });
}
