{ dockerTools, buildEnv, tuat-feed-server }: dockerTools.buildImage {
  name = "tuat-feed-server";
  tag = "latest";

  copyToRoot = buildEnv {
    name = "image-root";
    paths = [ tuat-feed-server ];
    pathsToLink = [ "/bin" ];
  };

  config.Cmd = [ "/bin/tuat-feed-server" ];
}
