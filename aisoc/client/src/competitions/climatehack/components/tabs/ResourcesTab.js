import perceiverIO from '../../assets/perceiver-io.png';
import './ResourcesTab.scss';

export default function ResourcesTab() {
  return <div className="ch-tab ch-resources-tab">
    <h2>Background Resources</h2>
    <p>
      Traditional solar PV forecasting methodologies have almost universally only used numerical weather predictions and solar electricity readings to forecast solar electricity output; however, numerical weather predictions are not especially useful for predicting solar irradiance. Climate Hack.AI looks to add satellite imagery to the input data set, and we briefly review the state of the art in this new domain.
    </p>
    <h3>Optical flow</h3>
    <p>
      A good baseline model is &quot;optical flow&quot;, which infers the movement of each pixel in the image from the most recent pair of images. It then uses those movement vectors to &quot;warp&quot; the most recent image to predict the future. It does surprisingly well! See this <a href="https://github.com/openclimatefix/predict_pv_yield/blob/main/notebooks/optical_flow_1.ipynb">Python code for computing optical flow</a> and a video of optical flow working on the Climate Hack.AI satellite dataset below.
    </p>

    <iframe src="https://www.youtube-nocookie.com/embed/LUixNoBCgqw" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>

    <p>
      There is also an interesting <a href="https://www.youtube.com/watch?v=5AUypv5BNbI">introduction to optical flow</a> on the <a href="https://www.youtube.com/channel/UC9-y-6csu5WGm29I7JiwpnA">Computerphile YouTube channel</a>.
    </p>

    <iframe src="https://www.youtube-nocookie.com/embed/5AUypv5BNbI" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>

    <h3>Google Research&apos;s MetNet & MetNet-2</h3>

    <p>
      Google Research&apos;s &quot;MetNet&quot; paper (<a href="https://arxiv.org/abs/2003.12140">Sønderby et al., 2020</a>) describes a deep neural network designed to predict precipitation up to eight hours into the future. MetNet performs better than state-of-the-art numerical weather prediction models and produces probabilistic outputs.
    </p>
    <p>
      The model does not use any numerical weather predictions in its inputs: instead, it takes as input a high-resolution image of the area of interest, as well as a larger, lower resolution, 1,024 km x 1,024 km context image that is large enough to capture any clouds that might cross the area of interest, from both precipitation radar and satellite imagery. Topographic maps are also included as extra channels in these images. This is then passed through a convolutional LSTM model, followed by a few axial attention layers to allow the model to learn what parts of the images are the most important.
    </p>
    <p>
      On 15th November 2021, Google Research announced <a href="https://ai.googleblog.com/2021/11/metnet-2-deep-learning-for-12-hour.html">MetNet-2</a>, which extends the prediction horizon to 12 hours by using input data with a significantly larger spatial extent (2,048 km &times; 2,048 km). This is a huge model that runs across 128 Google Cloud TPU v3 cores. To get the model to fit onto the hardware, the authors abandoned the axial-attention module that was present in MetNet-1.
    </p>

    <h3>DeepMind&apos;s deep generative models for skilful precipitation nowcasting</h3>

    <p>
      DeepMind has also been working on precipitation nowcasting. In 2021, DeepMind released their &quot;skilful precipitation nowcasting&quot; paper (<a href="https://www.nature.com/articles/s41586-021-03854-z">Ravuri et al., 2021</a>), which uses a generative adversarial network (GAN) to create realistic-looking precipitation nowcasts. Like the MetNet papers, the &quot;skilful nowcasting&quot; model does not look at numerical weather predictions.
    </p>
    <p>
      This model takes as input the last 20 minutes of imagery and outputs the next hour and a half of future imagery. It does this by creating four context stacks of images at different spatial resolutions, and passing them through a set of convolutional gated recurrent unit (GRU) layers that combine the context stacks at each level, starting from the smallest set of images, and working up to the largest images. These convolutional GRU layers are used to predict each timestep one at a time. As part of this process, a random vector is drawn from a uniform distribution and used as the initial hidden state of the bottom GRU layer, ensuring that each prediction is slightly different even with the same inputs, creating its probabilistic forecasts.
    </p>
    <p>
      Unlike the MetNet papers, the DeepMind authors put a lot of thought into figuring out if expert human weather forecasters &quot;believe&quot; the GAN&apos;s forecasts. They conclude that the DeepMind GAN produces predictions which humans find very believable.
    </p>
    <p>
      Open Climate Fix has implemented both <a href="https://github.com/openclimatefix/metnet">MetNet</a> and the <a href="https://github.com/openclimatefix/skillful_nowcasting">model in the skilful nowcasting paper</a>. See the <a href="https://docs.google.com/document/d/1vVmkGRxDkAKbFwfKbEWnN0-dToVEXQ6rPyUNT0C535w/edit#heading=h.m28qlf9c88cb">&quot;Architecture of OCF&apos;s ML models&quot; section</a> of their &quot;Solar PV Nowcasting Using Deep Learning&quot; report for more details.
    </p>

    <h3>DeepMind&apos;s Perceiver and Perceiver IO models</h3>

    <p>
      In the second half of 2021, DeepMind released two related papers, which have been highly influential on Open Climate Fix&apos;s approach to short-term solar forecasting: The &quot;Perceiver&quot; paper (<a href="https://arxiv.org/abs/2103.03206">Jaegle et al., 2021</a>), and the &quot;Perceiver IO&quot; paper (<a href="https://arxiv.org/abs/2107.14795">Jaegle et al., 2021b</a>). The Perceiver IO architecture (<a href="https://arxiv.org/abs/2107.14795">Jaegle et al., 2021b</a>) is shown below.
    </p>

    <img src={perceiverIO} />

    <p>
      The Perceiver is based on self-attention, which has proven to be an extremely powerful model for many domains. Recent breakthroughs such as OpenAI&apos;s GPT-3 and DeepMind&apos;s AlphaFold-2 use self-attention extensively.
    </p>
    <p>
      Conventional self-attention models suffer because their computational complexity goes up with the square of the length of the input (in this case the length of the input is linked to the number of pixels in the satellite and/or NWP data). This makes working on inputs such as large images intractable. The Perceiver introduces a beautifully simple way to limit the computational complexity of self-attention, thereby allowing these models to be applied to images and even videos.
    </p>
    <p>
      Open Climate Fix are particularly excited about the Perceiver because it excels at &quot;multi-modal&quot; tasks, such as being able to combine multiple different types of input (satellite imagery, numerical weather predictions, etc), which may be on different spatial or time grids; and also to perform multiple tasks (such as predicting GSP-level PV, PV for single PV systems or future satellite imagery).
    </p>
    <p>
      Furthermore, the input modalities do not need to be perfectly aligned in space and time. So, for example, hourly numerical weather predictions on a 2 km grid and 5-minutely satellite images on a 2-6 km grid can be input natively into the model without preprocessing or interpolating the data.
    </p>
    <p>
      The following are two implementations of the Perceiver:
    </p>
    <ul>
      <li><a href="https://github.com/openclimatefix/perceiver-pytorch">https://github.com/openclimatefix/perceiver-pytorch</a></li>
      <li><a href="https://huggingface.co/blog/perceiver">https://huggingface.co/blog/perceiver</a></li>
    </ul>
    <p>
      It is also possible to create a custom Perceiver IO model in just a few lines by using PyTorch&apos;s in-built <a href="https://pytorch.org/tutorials/beginner/transformer_tutorial.html">Transformer building blocks</a>.
    </p>
  </div>;
}
