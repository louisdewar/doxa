

export default function GettingStartedGuideTab() {
  return <div className="ch-tab">
    <h2>Getting Started Guide</h2>
    <p>
      Ensure you have Python 3.9 installed on your computer before you begin, as this is what the evaluation environment uses. Windows and macOS users may conveniently download it from the <a href="https://www.python.org/downloads/release/python-399/">Python website</a>.
    </p>
    <h3>Introductory tutorial</h3>
    <p>
      To help you get started, the development team at the UCL Artificial Intelligence Society have put together a short explainer video to guide you through making your first Climate Hack.AI submission to DOXA.
    </p>
    <iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/c_NkwHGHwks" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>
    <p>
      The tutorial shows how to open the Jupyter notebook in <a href="https://code.visualstudio.com/">Visual Studio Code</a>. If you want to follow along and do not have it installed, download and install it, as well as the <a href="https://marketplace.visualstudio.com/items?itemName=ms-python.python">Python extension</a>.
    </p>
    <h3>Downloading the example project</h3>
    <p>
      To get started, download and extract the example project from <a href="https://github.com/louisdewar/doxa/releases/latest/download/climatehack_getting_started.zip">GitHub</a>. In addition to containing more information about the dataset and the challenge, the Jupyter notebook in <code>training.ipynb</code> guides you through training a basic model and making your first Climate Hack.AI submission to DOXA.
    </p>
    <p>
      Also, make sure to check out Open Climate Fix&apos;s <a href="https://github.com/openclimatefix/Satip/blob/main/notebooks/load_and_plot_HRV_UK_Zarr_from_GCS.ipynb">notebook</a>, which provides a more in-depth introduction to reading the satellite data, as well as more technical information about the dataset.
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
    <h3>The evaluation environment</h3>
    <p>
      DOXA evaluates submissions inside a lightweight Linux-based virtual machine with 4 vCPUs and 6GB RAM. Submissions may be no larger than 4GB in size. The Python 3.9 packages installed in the evaluation environment include the following:
    </p>
    <pre>
      cachetools (5.0.0),
      dm-reverb (0.6.1),
      dm-tree (0.1.6),
      flatbuffers (2.0),
      gym (0.21.0),
      joblib (1.1.0),
      keras (2.8.0),
      Keras-Preprocessing (1.1.2),
      numba (0.55.1),
      numpy (1.21.5),
      opencv-contrib-python-headless (4.5.5.62),
      opt-einsum (3.3.0),
      einops (0.4.0),
      perceiver-pytorch (0.8.3),
      pandas (1.4.0),
      Pillow (9.0.1),
      scikit-learn (1.0.2),
      scipy (1.7.3),
      tensorboard (2.8.0),
      tensorflow (2.8.0),
      tf-agents (0.7.1),
      torch (1.10.2+cpu),
      torchaudio (0.10.2+cpu),
      torchvision (0.11.3+cpu),
      wrapt (1.13.3)
    </pre>
    <p>
      If there is a package you want to use that is not installed in the evaluation environment, reach out to us on <a href="https://discord.gg/HTTQ8AFjJp">Discord</a> and we will try our best to accommodate any reasonable requests.
    </p>
  </div>;
}
