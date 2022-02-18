import Card from './Card';


export default function VerifyEmailCard({ startLetter, domain }) {
  return <Card>
    <h2>Verify your email</h2>

    <p>
      Before you can use your account, you must first verify your email address.
    </p>

    <p>
      A verification code has been sent to <code>{startLetter}***@{domain}</code>.
    </p>

    <p>
      Please check your inbox (or spam folder) for an email from DOXA.
    </p>
  </Card>;
}
