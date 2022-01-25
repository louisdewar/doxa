import Card from 'components/Card';

function verifyEmail({ startLetter, domain }) {
  return <Card>
    <h2>Verify your email!</h2>
    
    <p>
      Before you can use your account you need to verify your email.
      Check your inbox (or spam folder) for an email from DOXA.
    </p>


    <p>
      The verification code was sent to <code>{startLetter}***@{domain}</code> (parts of
      email redacted for privacy).
    </p>
  </Card>;
}

export default function(incompleteFlowName, payload) {
  console.log(incompleteFlowName, payload);
  if (incompleteFlowName === 'verify_email') {
    return verifyEmail({ startLetter: payload.start_letter, domain: payload.domain });
  } else {
    return null;
  }
}