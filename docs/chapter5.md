# 第五章 上线


我们已经有了一个运行良好的新闻通讯API原型，现在是时候将其上线了。

我们将学习如何将我们的Rust应用程序打包为Docker容器，以便在[DigitalOcean](https://docs.digitalocean.com/products/app-platform/)的App平台上部署(译者， 这里我将选择别的一个平台进行部署，而不是DigitalOcean)。

在本章的最后，我们将拥有一个持续部署（CD）流水线：每次提交到主分支都会自动触发部署最新版本的应用程序给我们的用户。


## 5.1 我们必须谈论部署

每个人都喜欢谈论尽可能频繁地将软件部署到生产环境的重要性（我也是其中之一！）。

> "尽早获得客户反馈！"
> "经常发布并对产品进行迭代！"

但是没有人告诉你如何做到这一点。

随机选择一本关于Web开发或某个框架介绍的书。
大多数书对部署的主题只会用一段话来简单提及。
少数书籍可能会有一章来讨论部署，通常是在书的结尾，你可能根本没有机会真正阅读到。

只有少数书籍在合理的情况下，给予了它应有的重视。

为什么呢？

因为部署（至今）仍然是一项复杂的任务。

有很多供应商，大多数并不直观易用，而且被认为是最先进或最佳实践的事物往往变化迅速{1Kubernetes is six years old, Docker itself is just seven years old!}。

这就是为什么大多数作者避开这个话题的原因：它需要很多页的篇幅，而且很痛苦地写下来后才意识到，一两年后它已经过时了。

尽管如此，部署是软件工程师日常工作中重要的关注点——例如，谈论数据库架构迁移、领域验证和API演进时都很难不考虑你的部署流程。

在一本名为《从零到生产》的书中，我们简单不能忽视这个主题。


## 5.2 选择工具

本章的目的是让你亲身体验每次提交到主分支时的部署过程。

这就是为什么我们在第五章早早地谈论部署的原因：为了让你有机会在本书的其余部分实践这项技能，就像你在一个真实的商业项目中所做的那样。

实际上，我们特别关注的是持续部署工程实践对我们的设计选择和开发习惯的影响。

同时，构建完美的持续部署流水线并不是本书的重点，它需要一本独立的书籍，甚至可能需要整个公司的介入。

我们必须务实并在内在的有用性（即学习一个在行业中受到重视的工具）和开发者体验之间取得平衡。

即使我们花时间来打造“最佳”设置，由于你组织的具体限制，你仍然有可能选择不同的工具和不同的供应商。

重要的是其潜在的理念，并让你尝试将持续部署作为一种实践。

### 5.2.1 虚拟化：Docker

我们的本地开发环境和生产环境有着截然不同的目的。

浏览器、集成开发环境、音乐播放列表等可以共存于我们的本地机器上。它是一个多功能的工作站。
相比之下，生产环境的焦点更加狭窄：运行我们的软件，使其对用户可用。任何与此目标无关的东西，最好情况下都是资源浪费，最坏情况下是安全风险。

这种差异在历史上使得部署变得非常麻烦，引发了“在我的机器上可以运行”的抱怨，现在已经成为一个名言。

仅仅将源代码复制到我们的生产服务器是不够的。我们的软件很可能对底层操作系统提供的功能有所假设（例如，原生Windows应用程序无法在Linux上运行），对同一台机器上其他软件的可用性有所依赖（例如，某个版本的Python解释器），或者对其配置有所依赖（例如，我是否具备root权限）。

即使我们从两个相同的环境开始，随着版本的漂移和微妙的不一致性的出现，随着时间的推移，我们也会遇到问题。

确保我们的软件正确运行的最简单方法是严格控制其执行环境。

这是虚拟化技术的基本理念：如果你可以将一个包含应用程序的自包含环境部署到生产环境中，而不是仅仅部署代码，那将会非常好！

对于开发人员来说，这减少了周五晚上的意外；对于负责生产基础设施的人员来说，它提供了一个一致的抽象层。

如果环境本身可以作为代码来指定，以确保可重现性，那就更好了。

虚拟化的好处在于它已经存在，并且已经成为主流技术已经有将近十年的时间了。

对于技术中的大多数事物来说，根据你的需求，你有几个选择：虚拟机、容器（如Docker）和其他一些选项（如Firecracker）。

我们将选择主流且无处不在的选项 - Docker容器。

### 5.2.2 托管：DigitalOcean

[AWS](https://aws.amazon.com/)、[Google Cloud](https://cloud.google.com/?hl=zh-cn)、[Azure](https://azure.microsoft.com/en-gb/)、[Digital Ocean](https://www.digitalocean.com/)、[Clever Cloud](https://www.clever-cloud.com/)、[Heroku](https://www.heroku.com/)、[Qovery](https://www.qovery.com/)...
可以选择托管软件的供应商的列表还在继续增长。

人们已经成功地通过推荐最适合您特定需求和用例的云平台来创造了一个业务 -
这不是我的工作（尚未）或本书的目的。

我们正在寻找的是易于使用的东西（出色的开发者体验，最小的不必要复杂性）和相对成熟的解决方案。
在2020年11月，这两个要求的交集似乎是Digital Ocean，尤其是他们新推出的App Platform提案。

> 免责声明：Digital Ocean并没有支付我在这里推广他们的服务。

## 5.3 为我们的应用程序编写 Dockerfile


[DigitalOcean的App Platform](https://www.digitalocean.com/docs/app-platform/languages-frameworks/docker/)原生支持部署容器化应用程序。
这将是我们的第一个任务：我们需要编写一个Dockerfile来构建和执行我们的应用程序作为一个Docker容器。

### 5.3.1 Dockerfile

Dockerfile是您的应用程序环境的配方。

它们以层次结构组织：您从一个基础镜像开始（通常是一个包含编程语言工具链的操作系统），然后按顺序执行一系列命令（如COPY、RUN等），以构建所需的环境。

让我们来看一个最简单的用于Rust项目的Dockerfile示例：

```dockerfile
# We use the latest Rust stable release as base image
FROM rust:1.59.0
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
```

将其保存为名为 Dockerfile 的文件，放在我们的 git 存储库的根目录中。

```bash
zero2prod/
    .github/
    migrations/
    scripts/
    src/
    tests/
    .gitignore
    Cargo.lock
    Cargo.toml
    configuration.yaml
    Dockerfile
```

使用 Docker CLI 执行这些命令来获取镜像的过程称为构建。

```bash
# Build a docker image tagged as "zero2prod" according to the recipe
# specified in `Dockerfile`
docker build --tag zero2prod --file Dockerfile .
```

在命令末尾的点号 (.) 表示构建上下文，它指定了构建过程中的文件路径。在这种情况下，点号表示当前目录作为构建上下文，Docker 将会在当前目录中查找 Dockerfile 和其他需要复制到镜像中的文件。

### 5.3.2  Build Context

docker build 通过使用一个构建上下文来生成镜像。你可以将正在构建的Docker镜像想象为一个完全隔离的环境。镜像与你的本地机器之间唯一的联系点是像COPY或ADD这样的命令。构建上下文决定了在Docker容器内部，COPY和其他相关命令可以看到你本地机器上的哪些文件。

通过使用`.`，我们告诉Docker将当前目录作为这个镜像的构建上下文。因此，`COPY . app`命令将会将当前目录中的所有文件（包括我们的源代码！）复制到Docker镜像的`app`目录中。

使用`.`作为构建上下文意味着Docker不会允许`COPY`命令看到父目录中的文件，也不会允许它看到你本地机器上的任意路径中的文件。

根据你的需求，你可以使用不同的路径甚至是URL作为构建上下文。

### 5.3.3  Sqlx offline mode

这里我们使用的是sqlx的0.7版本，这里offline已经是默认支持的feature了，所以不需要再指定。


todo!()


## 5.4 部署到DigitalOcean Apps平台

我们已经构建了一个（非常好的）容器化版本的应用程序。现在让我们来部署它！

### 5.4.1 设置

您需要在[Digital Ocean的网站上注册账号](https://cloud.digitalocean.com/registrations/new)。

一旦您拥有了账号，请安装Digital Ocean的命令行工具doctl - 您可以在[此处](https://docs.digitalocean.com/reference/doctl/how-to/install/)找到安装说明。

> 在Digital Ocean的应用平台上进行托管并不是免费的 - 让我们的应用程序和关联的数据库保持运行大约需要每月20.00美元的费用。
>我建议您在每个会话结束时销毁该应用程序 - 这样可以保持您的花费远低于1.00美元。在我撰写本章时，我在尝试中只花费了0.20美元！


### 5.4.2 应用程序规范

Digital Ocean的应用平台使用一个声明性配置文件来让我们指定应用程序部署的样子 - 他们称之为[App Spec](https://www.digitalocean.com/docs/app-platform/concepts/app-spec/)。

根据参考文档和一些示例，我们可以组合出我们的App Spec的初步草稿。

让我们将这个清单文件，spec.yaml，放在项目目录的根目录下。

```yaml
#! spec.yaml
name: zero2prod
# Check https://www.digitalocean.com/docs/app-platform/#regional-availability
# for a list of all the available options.
# You can get region slugs from
# https://www.digitalocean.com/docs/platform/availability-matrix/
# They must specified lowercased.
# `fra` stands for Frankfurt (Germany - EU)
region: fra
services:
  - name: zero2prod
    # Relative to the repository root
    dockerfile_path: Dockerfile
    source_dir: .
    github:
      # Depending on when you created the repository,
      # the default branch on GitHub might have been named `master`
      branch: main
      # Deploy a new version on every commit to `main`!
      # Continuous Deployment, here we come!
      deploy_on_push: true
      # !!! Fill in with your details
      # e.g. LukeMathWalker/zero-to-production
      repo: DaviRain-Su/zero2prod
    # Active probe used by DigitalOcean's to ensure our application is healthy
    health_check:
      # The path to our health check endpoint!
      # It turned out to be useful in the end!
      http_path: /health_check
    # The port the application will be listening on for incoming requests
    # It should match what we specified in our configuration/production.yaml file!
    http_port: 8000
    # For production workloads we'd go for at least two!
    # But let's try to keep the bill under control for now...
    instance_count: 1
    instance_size_slug: basic-xxs
    # All incoming requests should be routed to our app
    routes:
      - path: /
```

请花一些时间熟悉所有指定的值，并了解它们的用途。
我们可以使用他们的 CLI，doctl，来第一次创建应用程序：

```bash
doctl apps create --spec spec.yaml
```

```bash
Error: Unable to initialize DigitalOcean API client: access token is required.
(hint: run 'doctl auth init')

错误：无法初始化 DigitalOcean API 客户端：需要访问令牌。
（提示：运行“doctl auth init”）
```

好吧，我们必须先进行身份验证。
让我们按照他们的建议来做。

```bash
doctl auth init
```

```bash
Please authenticate doctl for use with your DigitalOcean account.
You can generate a token in the control panel at
https://cloud.digitalocean.com/account/api/tokens
```

一旦你提供你的令牌，我们可以再试一次：

```bash
doctl apps create --spec spec.yaml
```

```bash
Error: POST
https://api.digitalocean.com/v2/apps: 400 GitHub user not
authenticated
```

好的，按照他们的说明来链接你的 GitHub 帐户。
第三次是魅力，让我们再试一次！

```bash
doctl apps create --spec spec.yaml
```

```bash
Notice: App created
ID Spec Name Default Ingress Active Deployment ID In Progress Deployment ID
e80... zero2prod
```

它成功了！
你可以用以下命令来检查你的应用程序状态：

```bash
doctl apps list
```

或者通过查看 [DigitalOcean 的仪表板]()。

虽然应用程序已经成功创建，但它还没有运行！

检查他们的仪表板上的部署选项卡 - 它可能正在构建 Docker 镜像。

根据他们错误跟踪器上的[最近几个问题]()，它可能需要一段时间 -
有几人报告说他们遇到了缓慢的构建。Digital Ocean 的支持工程师建议利用 Docker 层缓存来缓解该问题 - 我们已经在那里覆盖了所有基础！

>如果您在 DigitalOcean 上构建 Docker 镜像时遇到内存不足错误，请查看此 [GitHub 问题]()。


等待这些行显示在他们的仪表板构建日志中：

```bash
zero2prod | 00:00:20 => Uploaded the built image to the container registry
zero2prod | 00:00:20 => Build complete
```

部署成功！
您应该能够每隔 10 秒或更短的时间看到健康检查日志，当 DigitalOcean 的平台向我们的应用程序发送 ping 来确保它正在运行。

```bash
doctl apps list
```

您可以检索您应用程序的面向公众的 URI。类似以下内容：

```bash
https://zero2prod-aaaaa.ondigitalocean.app
```

尝试现在发送一个健康检查请求，它应该返回一个 200 OK！

注意，DigitalOcean 已经为我们设置了 HTTPS，通过提供证书并将 HTTPS 流量重定向到我们在应用程序规范中指定的端口。 少操心一件事。
POST /subscriptions 端点仍然失败，与本地完全相同的方式：我们没有在生产环境中为我们的应用程序提供实时数据库。
让我们提供一个。

将此部分添加到您的 spec.yaml 文件中：

```yaml
databases:
  # PG = Postgres
  - engine: PG
    # Database name
    name: newsletter
    # Again, let's keep the bill lean
    num_nodes: 1
    size: db-s-dev-database
    # Postgres version - using the latest here
    version: "12"
```

然后更新您的应用程序规范：

```bash
# You can retrieve your app id using `doctl apps list`
doctl apps update YOUR-APP-ID --spec=spec.yaml
```

DigitalOcean 需要一些时间来创建 PostgreSQL 实例。
在此期间，我们需要弄清楚如何将我们的应用程序指向生产环境中的数据库。

### 5.4.3 如何使用环境变量注入密码

连接字符串将包含我们不想提交到版本控制的值，例如我们数据库的 root 用户的用户名和密码。
我们最好的选择是使用环境变量作为在运行时将密码注入到应用程序环境中的方法。例如，DigitalOcean 的应用程序可以引用 DATABASE_URL 环境变量（或其他几个变量以获得更细粒度的视图）来在运行时获取数据库连接字符串。
我们需要升级我们的 get_configuration 函数（再次）来满足我们新的 要求。

```rust

```

这允许我们使用环境变量自定义 Settings 结构中的任何值，覆盖配置文件中指定的值。
为什么这很方便？

它使我们可以注入太动态（即事先不知道）或太敏感而无法存储在版本控制中。

它还使我们能够快速更改我们的应用程序的行为：如果我们想调整其中一个值（例如数据库端口），我们不必进行完整的重建。对于像 Rust 这样的语言，如果一个新的构建需要十分钟或更长时间，这可能意味着在短暂的停机和对客户可见的影响之间有很大差异。

在继续之前，让我们来处理一个令人烦恼的细节：环境变量对于 config 包来说是字符串，如果使用 serde 的标准反序列化例程，它将无法接收整数。

幸运的是，我们可以指定一个自定义的反序列化函数。

让我们添加一个新的依赖项，serde-aux（serde 辅助）
