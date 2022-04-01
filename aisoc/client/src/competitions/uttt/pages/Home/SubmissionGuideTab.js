import './SubmissionGuideTab.scss';

export default function SubmissionGuideTab() {
  return <div className="submission-guide-tab">
    <h2>Submission Guide</h2>
    <p>
      Ensure you have Python 3.9 installed on your computer before you begin.
    </p>
    <p>
      To get started, download and extract the example agent project from <a href="https://github.com/louisdewar/doxa/releases/download/0.1.2/uttt_getting_started.zip">GitHub</a>.
    </p>
    <h3>Using the Doxa CLI</h3>
    <p>
      To use the Doxa CLI, run the following command from the extracted folder:
    </p>
    <pre>
      python doxa_cli.py --help
    </pre>
    <p>
      <strong>Note</strong>: on some systems, you may have to type <code>python3</code> instead of <code>python</code>.
    </p>
    <p>
      If you do not have the relevant Doxa CLI binary for your operating system, it will be downloaded into the <code>{'"bin"'}</code> folder.
    </p>
    <p>
      Before you can submit your agent code, you must first log in using the following command:
    </p>
    <pre>
      python doxa_cli.py user login
    </pre>
    <p>
      When you want to upload your agent, run the following command:
    </p>
    <pre>
      python doxa_cli.py agent upload uttt ./agent
    </pre>
    <p>
      Here, <code>{'"./agent"'}</code> refers to the folder containing the agent code you wish to submit.
    </p>
  </div>;
}
