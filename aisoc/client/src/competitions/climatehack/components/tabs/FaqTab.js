

export default function FaqTab() {
  return <div className="ch-tab">
    <h2>Frequently Asked Questions</h2>
    <p>
      If you have a question that has not been answered here, ask us on <a href="https://discord.gg/HTTQ8AFjJp">Discord</a>, where we have several help and FAQ channels!
    </p>
    <h3>Competition queries</h3>
    <h4>&rsaquo; Can we participate in teams?</h4>
    <p>
      You can compete in the first round either as an individual or as a team of any size. We would encourage you to collaborate widely, including with other individuals and teams! Having said that, only the top three competitiors from each society may proceed to represent their university in the finals, so you may want to limit the size of your team to three.
    </p>
    <p>
      The university society of which you are a member may issue additional guidelines, depending on how they choose to support you locally. For example, they may have a policy of encouraging everyone to form teams of three. While these are only guidelines, we recommend that you follow them nevertheless.
    </p>
    <h3>Common submission issues</h3>
    <h4>&rsaquo; I don&apos;t see my agent on the leaderboard after uploading it. What should I do?</h4>
    <p>
      If this happens, there was most likely an error processing your submission. If you are logged in on DOXA, you can view the <code>stderr</code> output of your agent by clicking &lsquo;Your submission&rsquo; in the leaderboard tab. Alternatively, you can view your latest submission from the account page.
    </p>
    <h4>&rsaquo; How can I fix a &lsquo;wrong output type&rsquo; error?</h4>
    <p>
      DOXA expects your code to produce NumPy arrays of type <code>numpy.float32</code>. If you return an array of type <code>numpy.float64</code> or anything else, DOXA cannot grade your submission.
    </p>
    <p>
      You can usually fix this by casting your output NumPy array to the right type, e.g. by running something equivalent to <code>output.astype(numpy.float32)</code>. Recall that your model should output pixel values in the range 0.0 to 1023.0 (inclusive).
    </p>
    <h4>&rsaquo; My agent crashes with &lsquo;Attempting to deserialize object on a CUDA device but torch.cuda.is_available() is False&rsquo;. What can I do?</h4>
    <p>
      The evaluation environment does not have a CUDA-enabled GPU, so attempting to deserialise the model with <code>load_state_dict</code> will fail. To remedy this, you can replace your <code>torch.load(&apos;model.pt&apos;)</code> call (or equivalent) with the following:
    </p>
    <pre>
      torch.load(&apos;model.pt&apos;, map_location=torch.device(&apos;cpu&apos;))
    </pre>
  </div>;
}
