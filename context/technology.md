# Technology and infrastructure

Profile version: 1.0.0 · Reviewed: 10-18-2026 · America/New_York

Rendered from [../profile.json](../profile.json). Dates are evidence dates, not live verification. See [../SOURCES.md](../SOURCES.md) and [../UNKNOWNS.md](../UNKNOWNS.md).

## Technology and infrastructure

<a id="record-tech-priorities"></a>

- **tech-priorities** — Homelab priority order: efficiency, then performance, then sound. Reliability, maintainability and real operating cost matter more than chasing specifications.
  
  preferred; active; documented/high; last supported: unknown; sources: S02, S05

<a id="record-tech-unifi"></a>

- **tech-unifi** — UniFi is the chosen direction for new production networking, with VLAN separation and multigigabit RJ45 where appropriate. This is a design preference, not proof that UniFi hardware is installed.
  
  required; active; documented/high; last supported: unknown; sources: S02, S05, S08

<a id="record-tech-local"></a>

- **tech-local** — Prefer private local camera networks, local storage/control and private VPN access for administration. Keep production separate from experiments and use least privilege.
  
  required; active; retrieved_explicit/high; last supported: unknown; sources: S03, S05

<a id="record-tech-preserve"></a>

- **tech-preserve** — Preserve working networking, storage, backups and the Windows fallback until a replacement has passed relevant compatibility and recovery checks. General preferences do not authorize a live migration or public exposure of admin services.
  
  required; active; retrieved_explicit/high; last supported: unknown; sources: S03, S05

<a id="record-tech-linux"></a>

- **tech-linux** — Prefer a low-maintenance system that works without continual repair. Earlier server briefs required Ubuntu/LTS; newer focused records permit Fedora/Cockpit/KVM as a compute/lab candidate. Keep those roles distinct. Mint Cinnamon leads the saved desktop shortlist, with Kubuntu/Fedora KDE alternatives; no installation is implied.
  
  preferred; active; documented/high; last supported: unknown; sources: S10, S11

<a id="record-tech-compat"></a>

- **tech-compat** — Check the actual OS, architecture, versions, GPU/backend, required apps, games/anti-cheat, VR, HDR/VRR, sleep/wake and displays before declaring a migration compatible.
  
  required; active; documented/high; last supported: unknown; sources: S05, S07

<a id="record-tech-browser"></a>

- **tech-browser** — Every existing browser extension is required for a Brave/Firefox migration; verify exact functionality and Manifest V3 impact. The complete extension list is not in this repo, so do not claim parity or invent it.
  
  required; active; documented/high; last supported: unknown; sources: S02, S05

<a id="record-tech-futo"></a>

- **tech-futo** — For FUTO Keyboard research, focus on autocorrect, prediction, punctuation and everyday reliability. Do not introduce swipe typing.
  
  required; active; documented/high; last supported: unknown; sources: S02, S05

<a id="record-tech-vr"></a>

- **tech-vr** — For VR purchases, OLED is a hard requirement. Check comfort and wireless needs for the specific task.
  
  required; active; documented/high; last supported: unknown; sources: S02, S05

<a id="record-tech-calibration"></a>

- **tech-calibration** — Display accuracy, calibration, HDR/VRR and audio are recurring interests. Use the owned displays/audio as the baseline and verify exact model behavior.
  
  preferred; active; documented/high; last supported: unknown; sources: S02, S07

<a id="record-tech-gui"></a>

- **tech-gui** — Prefer polished interfaces, clear guided procedures, broad maintenance/community support and low routine upkeep. Technical experience does not mean Justin wants to babysit terminals or use fringe, poorly maintained systems.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10

<a id="record-tech-mgig"></a>

- **tech-mgig** — The Ethernet preference is multigigabit RJ45 with 1/2.5/5/10 Gb/s support, not a 10G-only port. SFP+ is less convenient; consider it only with exact transceiver compatibility, power, cost and whole-layout value established.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10

<a id="record-tech-network-survey"></a>

- **tech-network-survey** — Whole-home roaming, indoor/outdoor coverage and room for cameras matter. Survey ports, PoE, cabling, RF coverage and recording needs before choosing a fixed bill of materials; retain the existing network as rollback.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10, S11

<a id="record-tech-zones"></a>

- **tech-zones** — The saved target zones are Management, Trusted, Servers, IoT, Cameras, Guest and Lab/AI. Use explicit least-privilege flows, equivalent IPv4/IPv6 controls and only necessary discovery exceptions; do not assume VLANs are deployed.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-no-public-admin"></a>

- **tech-no-public-admin** — Prefer private VPN access over routine public port forwarding. Do not assume any VPN/tunnel, NAT setting or alternate GUI port proves isolation; verify the actual reachable services and boundary.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10, S11

<a id="record-tech-compose"></a>

- **tech-compose** — Versioned external Compose/configuration is the intended authority; Portainer is an interface. Recover existing definitions and data before rebuilding. Blind automatic infrastructure updates are outside the target architecture.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-storage-boundary"></a>

- **tech-storage-boundary** — Keep Synology production storage while capacity is stabilized. Pilot the already-owned HexOS/TrueNAS direction on expendable media; prove backup, restore, power and operational behavior before migration. Do not treat snapshots or a new array as an independent backup.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-ecc"></a>

- **tech-ecc** — ECC, expansion lanes, sufficient memory, hardware encoding/transcoding, drive growth and UPS behavior are important server-design goals. Verify actual ECC reporting, headless reliability, wall power and final installed parts rather than assuming a compatible build.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10, S11

<a id="record-tech-server-historical"></a>

- **tech-server-historical** — Earlier server briefs specified a Define 7 XL, air cooling, at least 1000 W and Titanium-class PSU efficiency. These are historical design targets, not measured load or a locked current BOM; reassess the exact build under current efficiency/TCO gates.
  
  context; historical; archive_explicit/high; last supported: unknown; sources: S10

<a id="record-tech-control-plane"></a>

- **tech-control-plane** — Home Assistant is the intended offline automation authority; Alexa/Echo is optional convenience. Core manual controls, deterministic home functions, recording and storage must not depend on an LLM or internet availability.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-ai-measure"></a>

- **tech-ai-measure** — Use the owned RX 7900 XTX as the on-demand local-AI baseline before buying around an unmeasured bottleneck. The saved direction is Ollama/Open WebUI, with OpenCode replaceable and Odysseus experimental; verify current compatibility and actual workload results.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-agent-sandbox"></a>

- **tech-agent-sandbox** — Agent/computer-use experiments belong in controlled disposable environments with scoped files and permissions. Do not install broad-access experiments on daily-use devices by default or grant production Docker/NAS/router authority from a general AI preference.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10, S11

<a id="record-tech-monitoring-minimal"></a>

- **tech-monitoring-minimal** — Start with native alerts and the smallest monitoring stack that identifies outages, failures and changes. AI may summarize sanitized evidence; it is not the monitor or an autonomous production-remediation authority.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-game-versions"></a>

- **tech-game-versions** — Historical game/modpack-server requests prefer current supported releases and easy pack changes, avoiding unnecessary fixed-version locks. Scope that preference to games; preserve compatibility, backups and controlled updates for production services.
  
  preferred; active; archive_explicit/high; last supported: unknown; sources: S10

<a id="record-tech-wallpaper"></a>

- **tech-wallpaper** — Wallpaper Engine functionality is a standing requirement for desktop OS evaluation, including Linux/macOS alternatives. Verify the actual wallpaper and interaction features; do not equate a static image, partial plugin or claimed port with full parity.
  
  required; active; archive_explicit/high; last supported: unknown; sources: S10, S11

<a id="record-tech-media-quality"></a>

- **tech-media-quality** — Preserve original media quality where possible. Distinguish a remux/container change from re-encoding and do not promise lossless compression or working hardware acceleration without evidence for the actual files and stack.
  
  preferred; active; archive_explicit/high; last supported: unknown; sources: S10

<a id="record-tech-vr-mobility"></a>

- **tech-vr-mobility** — A primary Quest 3 replacement must combine OLED/micro-OLED, inside-out tracking without required base stations, and practical wireless PCVR across rooms away from the upstairs PC. A wired-only specialist headset or ordinary walking pad does not satisfy that mobility/locomotion goal.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-display-oled"></a>

- **tech-display-oled** — The saved focused display record specifies OLED for future display-led purchases unless Justin explicitly requests an exception. Preserve existing equipment; the laptop exception conflict has its own working rule and must not be silently generalized.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-display-path"></a>

- **tech-display-path** — The desktop Linux acceptance target includes the actual 7800X3D/7900 XTX/S90F path, 4K 144-class output, HDMI 2.1, HDR/10-bit quality, VRR, audio, sleep/wake, the real game/anti-cheat library, Wallpaper Engine and VR. Keep Windows available until the relevant real-use gates pass.
  
  required; active; documented/high; last supported: unknown; sources: S11

<a id="record-tech-capture-router"></a>

- **tech-capture-router** — A small deterministic capture/intent router is a saved design interest: phone/shortcut/webhook inputs, documented versioned data and idempotent routing before model decisions. n8n is optional orchestration; a cloud-edge lab is replaceable and must not hold broad home-admin authority.
  
  context; planned; documented/high; last supported: unknown; sources: S11

