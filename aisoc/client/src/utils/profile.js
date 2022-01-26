// TODO: maybe move this to a `models` folder and include other models for representing responses from API

const UNIVERSITIES = {
  'UNKNOWN': {
    name: 'Unknown',
    logo: null
  },
  'ucl.ac.uk': {
    name: 'UCL',
    logo: null
  }
};

export default class UserProfile {
  constructor(profile) {
    this.profile = profile;

    this._university = UNIVERSITIES[(profile.extra && profile.extra.org) || 'UNKNOWN'] || UNIVERSITIES['UNKNOWN'];
  }

  name() {
    return this.profile.username;
  }

  university() {
    console.log(this._university, (this.profile.extra && this.profile.extra.org) || 'UNKNOWN');
    return this._university;
  }
}
