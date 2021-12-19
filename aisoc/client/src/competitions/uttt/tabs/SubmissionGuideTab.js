import './SubmissionGuideTab.scss';

export default function SubmissionGuideTab() {
  return <div className="submission-guide-tab">
    <h2>Submission Guide</h2>
    <p>
      Ensure you have Python 3.7+ installed on your computer before you begin.
    </p>
    <p>
      To get started, download the example agent project from <a href="https://github.com/louisdewar/doxa/releases/download/0.0.1/uttt_getting_started.zip">GitHub</a>.
    </p>
    <h3>Using the Doxa CLI</h3>
    <p>
      To use the Doxa CLI, run the following command:
    </p>
    <pre>
      python doxa_cli.py --help
    </pre>
    <p>
      If you do not have the relevant Doxa CLI binary for your operating system, it will be downloaded into the {'"bin"'} folder.
    </p>
    <p>
      Before you can submit your agent code, you must first login using the following command:
    </p>
    <pre>
      python doxa_cli.py user login
    </pre>
    <p>
      When you want to submit, run the following command from the root of your agent folder:
    </p>
    <pre>
      python doxa_cli.py agent upload uttt ./agent
    </pre>
    <p>
      Here, {'"./agent"'} refers to the folder containing the agent code you wish to submit.
    </p>
  </div>;
}
