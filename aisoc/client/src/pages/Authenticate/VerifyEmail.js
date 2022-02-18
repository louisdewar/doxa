import { verifyEmail } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

export default function VerifyEmail() {
  const [error, setError] = useState(null);
  const [completed, setCompleted] = useState(false);
  const [nextStep, setNextStep] = useState(null);
  const auth = useAuth();


  useEffect(async () => {
    const queryParams = new URLSearchParams(window.location.search);
    const verificationCode = queryParams.get('verification_code');

    if (!verificationCode) {
      setError('The verification code is missing!');
      return;
    }

    try {
      const response = await verifyEmail(verificationCode);

      const authToken = response.authToken();

      if (authToken || auth.isLoggedIn()) {
        setNextStep(
          <Link to="/account">
            <Button success>Go to your account</Button>
          </Link>
        );
        // If the user is already logged in and there was an auth token returned then they will be
        // logged into that verified account
        if (authToken) {
          auth.setAuthToken(authToken);
        }
      } else {
        setNextStep(
          <Link to="/authenticate">
            <Button success>Login</Button>
          </Link>
        ); 
      }

      setCompleted(true);

    } catch (e) {
      let error_message = 'Uh oh! Something went wrong verifying your email';
      if (e instanceof DoxaError) {
        console.error(`Failed to login (${e.error_code}): ${e.error_message}`);
        error_message += `: ${e.error_message}.`;
      } else {
        console.error(`Failed to login: ${e}`);
        error_message += '.';
      }
      setError(error_message);
    }  
  }, []);
  
  if (error) {
    return <Card>
      <p>{error}</p>

      <br />

      <Link to="/">
        <Button failure>Return to the hompage</Button>
      </Link>
    </Card>;
  } else if(completed) {
    return <Card>
      <p>
        Your email has been verified! Welcome to DOXA.
      </p>

      {nextStep}
    </Card>;
  } else {
    return null;
  }
}