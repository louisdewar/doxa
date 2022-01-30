

export default function GettingStartedGuideTab() {
  return <div className="ch-tab">
    <h2>Getting Started Guide</h2>
    <p>
      Ensure you have Python 3.9 installed on your computer before you begin.
    </p>
    <h3>Getting Started Tutorial</h3>
    <p>
      To help you get started, the development team at the UCL Artificial Intelligence Society have put together a short explainer video to guide you through making your first Climate Hack.AI submission to DOXA.
    </p>
    <iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/c_NkwHGHwks" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>
    <h3>Downloading the example project</h3>
    <p>
      To get started, download and extract the example project from <a href="https://github.com/louisdewar/doxa/releases/latest/download/climatehack_getting_started.zip">GitHub</a>. The <code>training.ipynb</code> Python Jupyter notebook contains information on the dataset and how to train a basic model to submit to DOXA.
    </p>
    <p>
      Also, check our Open Climate Fix&apos;s <a href="https://github.com/openclimatefix/Satip/blob/main/notebooks/load_and_plot_HRV_UK_Zarr_from_GCS.ipynb">notebook</a> a more in-depth introduction to reading the satellite data and more technical details about the dataset.
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
      python doxa_cli.py agent upload climatehack ./submission
    </pre>
    <p>
      Here, <code>{'"./submission"'}</code> refers to the folder containing the code used to evaluate your trained model.
    </p>
  </div>;
}
