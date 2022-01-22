

export default function SubmissionGuideTab() {
  return <div className="ch-tab">
    <h2>Submission Guide</h2>
    <p>
      Ensure you have Python 3.7+ installed on your computer before you begin.
    </p>
    <p>
      To get started, download and extract the example project from <a href="#">GitHub</a>.
    </p>
    <h3>Using the Doxa CLI</h3>
    <p>
      To use the Doxa CLI, run the following command from the extracted folder:
    </p>
    <pre>
      python doxa_cli.py --help
    </pre>
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
      python doxa_cli.py agent upload climatehack ./evaluation
    </pre>
    <p>
      Here, <code>{'"./evaluation"'}</code> refers to the folder containing the code used to evaluate your trained model.
    </p>
  </div>;
}
