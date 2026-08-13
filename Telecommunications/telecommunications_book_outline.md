# Telecommunications — Master Outline

*The structural contract for the book. Everything downstream refers back to this
file. It is not built into the PDF.*

---

## VOICE SPEC

> The book is written in a **lyrical-historical** register: every technical idea
> is entered through the people, places, and pressures that produced it, and the
> mathematics arrives as the resolution of a human problem rather than as a
> preamble. Sentences run long and periodic where the narrative needs to breathe,
> and go short when a fact has to land. The narrator is a first-person-plural
> guide — *we* — who is unashamedly fond of engineers, skeptical of marketing,
> and who names dates, rooms, and numbers when they are known and never invents
> them when they are not. Metaphor is used generously but is always cashed out in
> arithmetic before the section ends: an analogy that cannot be converted into a
> calculation is a decoration, and this book does not decorate. Each chapter
> opens with a scene — Shannon in Murray Hill, an ALOHA packet crossing between
> Hawaiian islands, Metcalfe drafting a memo on a typewriter at PARC — and closes
> by returning to that scene with the reader now able to explain what happened
> and why it had to happen that way. Terminology is introduced twice: first as the
> answer to a problem the reader has already felt, then, in a marked **Network+
> note**, as the vocabulary the industry and the CompTIA N10-009 exam actually
> use. The reader is addressed directly and treated as a colleague in training,
> never as a student to be caught out.

Threaded conventions, held in every chapter:

- **The recurring question.** *How do we get information from one process on one
  computer to another process on another computer — reliably, efficiently,
  securely, and at scale?* Every concept in this book appears because it answers
  some part of that question. Chapters state which part.
- **Problem before name.** No term is defined before the reader has felt the
  problem it solves.
- **Network+ notes.** Set off in a blockquote, always *after* the derivation, so
  vocabulary attaches to understanding rather than replacing it.
- **Failure-mode boxes.** Every layer we build generates new ways to break. Each
  chapter ends its final section with "What breaks here" — the troubleshooting
  thread that runs the length of the book.
- **Real numbers only.** Cited standards, dates, measurements and people are
  real. Where a figure is illustrative it is labelled as such.

---

## Audience, Prerequisites, Notation

**Audience.** Undergraduates meeting networking for the first time, self-learners
preparing for CompTIA Network+ (N10-009), and working technicians who learned the
commands but never the reasons. No networking background is assumed. Binary
arithmetic is built from scratch in Chapter 2 and never assumed before that.

**Prerequisites.** Comfort with a computer's command line, high-school algebra,
and the willingness to convert a metaphor into a calculation.

**Notation conventions, fixed once here:**

| Convention | Choice |
|---|---|
| Data rate | bits per second, lowercase `b` (`Mb/s`), never `MB/s` unless storage is meant |
| Storage | bytes, uppercase `B` |
| Prefixes | decimal SI for rates (`1 Gb/s = 10⁹ b/s`), binary IEC where memory is meant (`GiB`) |
| Addresses | IPv4 dotted decimal with CIDR prefix (`192.168.10.70/27`); IPv6 lowercase, RFC 5952 compression |
| MAC addresses | colon-separated lowercase hex (`00:1b:44:11:3a:b7`) |
| Layer numbering | OSI numbering (L1–L7) used for diagnosis; TCP/IP four-layer names used for architecture |
| Frames/packets | *frame* at L2, *packet* at L3, *segment* (TCP) / *datagram* (UDP) at L4 — held strictly |
| Decibels | `dB` for ratios, `dBm` for absolute power referenced to 1 mW |
| Byte order | network byte order (big-endian) throughout |

---

## The Dependency Spine

The book is ordered by what a concept *requires*, not by historical or textbook
convention. The spine:

```
information → signals → media → sharing a medium → local delivery (Ethernet)
   → the idea of layers → global addressing (IP) → subnetting → routing
   → transport (TCP/UDP) → services (DNS/DHCP/HTTP) → wireless → wide area
   → operations → security → troubleshooting → design
```

Two deliberate departures from convention:

1. **The OSI model is Unit V, not Chapter 1.** Layering is taught only after the
   reader has personally hit four different problems that layering solves.
   Presented earlier it is a vocabulary list; presented here it is a relief.
2. **Troubleshooting is a thread, not a unit.** Unit XIII formalises a method the
   reader has already been using for twelve units.

---

## Unit I — The Signal and the Symbol

*Chapters 1–4. Directory: `book/unit_01_information_and_communication/`*

What information *is*, how it is counted, and the hard physical ceiling on how
fast it can be moved. This unit earns the word "telecommunications" in the title:
before anything is a network, it is a channel with a capacity.

**Ch 1 — What Networks Are For.** The problem of distance, from signal fires to
the first transatlantic cable; the anatomy of any communication system; hosts,
links and protocols; the recurring question that organises the book.
*Sections:* `s01_the_problem_of_distance`, `s02_anatomy_of_a_communication_system`,
`s03_hosts_links_and_protocols`, `s04_the_question_this_book_answers`

**Ch 2 — Bits, Bytes, and Bases.** Information as resolved choice; binary and
powers of two built from nothing; hexadecimal and why byte boundaries matter;
how meaning is layered onto bit patterns.
*Sections:* `s01_information_as_choice`, `s02_binary_and_powers_of_two`,
`s03_hexadecimal_and_byte_boundaries`, `s04_representing_meaning`

**Ch 3 — The Measures of a Network.** Bandwidth vs. throughput vs. goodput;
latency decomposed into propagation, transmission, queueing and processing;
jitter and loss; the bandwidth–delay product and why a fat fast pipe can sit idle.
*Sections:* `s01_bandwidth_and_throughput`, `s02_latency_and_its_components`,
`s03_jitter_and_loss`, `s04_bandwidth_delay_product`

**Ch 4 — Shannon's Limit.** Entropy and surprise; Nyquist's symbol-rate ceiling;
noise, SNR, and the decibel; the capacity theorem and what it forbids.
*Sections:* `s01_entropy_and_surprise`, `s02_nyquist_and_the_symbol_rate`,
`s03_noise_snr_and_decibels`, `s04_the_capacity_theorem`

---

## Unit II — Making Bits Travel

*Chapters 5–10. Directory: `book/unit_02_signals_and_transmission/`*

The physical layer, derived rather than catalogued. Every impairment in this unit
reappears in Unit XIII as a symptom.

**Ch 5 — Signals in Time and Frequency.** Analog and digital as representations,
not things; the frequency domain and why it is the useful one; the bandwidth of a
channel; baseband vs. broadband.
*Sections:* `s01_analog_and_digital`, `s02_the_frequency_domain`,
`s03_bandwidth_of_a_channel`, `s04_baseband_and_broadband`

**Ch 6 — Impairments.** Attenuation and the decibel budget; thermal, shot and
impulse noise; distortion and dispersion; crosstalk, EMI and the reason twisted
pair is twisted.
*Sections:* `s01_attenuation`, `s02_noise`, `s03_distortion_and_dispersion`,
`s04_crosstalk_and_interference`

**Ch 7 — Line Coding.** Why raw NRZ fails on a long link; self-clocking codes
(Manchester, differential Manchester); block codes 4B/5B, 8B/10B, 64B/66B and the
overhead arithmetic they imply; multilevel signalling and PAM-4/PAM-16.
*Sections:* `s01_why_raw_nrz_fails`, `s02_self_clocking_codes`,
`s03_block_codes`, `s04_multilevel_signaling_and_pam`

**Ch 8 — Modulation.** Carriers and keying (ASK/FSK/PSK); phase, quadrature and
the I/Q plane; QAM constellations and bits per symbol; OFDM and why every modern
radio uses it.
*Sections:* `s01_carriers_and_keying`, `s02_phase_and_quadrature`,
`s03_qam_and_constellations`, `s04_ofdm_and_modern_radio`

**Ch 9 — Multiplexing.** FDM and the birth of carrier telephony; TDM and the
digital hierarchy; statistical multiplexing as the packet network's core economic
argument; CDMA and WDM.
*Sections:* `s01_frequency_division`, `s02_time_division_and_the_digital_hierarchy`,
`s03_statistical_multiplexing`, `s04_code_and_wavelength_division`

**Ch 10 — Transmission Media.** Twisted pair and the category ladder; coax; the
physics of optical fibre, single vs. multimode; free space and spectrum;
a decision procedure for choosing a medium from distance, rate, environment,
and budget.
*Sections:* `s01_twisted_pair`, `s02_coaxial_cable`, `s03_optical_fiber`,
`s04_free_space_and_spectrum`, `s05_choosing_a_medium`

---

## Unit III — Many Machines, One Infrastructure

*Chapters 11–14. Directory: `book/unit_03_sharing_infrastructure/`*

The combinatorial problem of connecting *n* things, and the two great answers —
circuits and packets — told as the evolutionary path from the telephone exchange
to the Internet.

**Ch 11 — Topologies.** n(n−1)/2 and why full mesh dies; bus, ring, star, mesh
and hybrid; physical vs. logical topology; hierarchy and the access/distribution/
core model.
*Sections:* `s01_the_combinatorics_of_connection`, `s02_bus_ring_star_mesh`,
`s03_physical_versus_logical`, `s04_hierarchy_and_the_three_tier_model`

**Ch 12 — Circuit Switching and the PSTN.** The operator and the crossbar;
digitisation, PCM and the 64 kb/s DS0; SS7 and out-of-band signalling; Erlang's
blocking formula and the economics of dimensioning. Taught as *why packets
happened*, not as a legacy catalogue.
*Sections:* `s01_the_operator_and_the_crossbar`, `s02_digitization_and_the_t_carrier`,
`s03_signaling_and_ss7`, `s04_erlangs_and_blocking`

**Ch 13 — Packet Switching.** Baran, Davies and the distributed-network papers;
datagrams vs. virtual circuits; store-and-forward and the queue; the efficiency
argument that ended the circuit era.
*Sections:* `s01_barans_distributed_network`, `s02_datagrams_and_virtual_circuits`,
`s03_store_and_forward_and_queueing`, `s04_why_packets_won`

**Ch 14 — LANs, WANs, and the Internet.** Scope as an engineering variable
(PAN/LAN/CAN/MAN/WAN); the network of networks; client–server, peer-to-peer and
the cloud's re-centralisation; convergence of voice, video and data on one fabric.
*Sections:* `s01_scopes`, `s02_the_network_of_networks`,
`s03_clients_servers_and_peers`, `s04_convergence`

---

## Unit IV — Local Delivery

*Chapters 15–20. Directory: `book/unit_04_local_delivery_ethernet/`*

The first complete working network the reader can build: two or more machines on
one segment, talking.

**Ch 15 — Frames and Hardware Addresses.** Why data is chopped into frames;
the MAC address, the OUI, and administered vs. burned-in addressing; the Ethernet
II frame field by field; CRC-32 and the FCS.
*Sections:* `s01_why_frames_exist`, `s02_the_mac_address`, `s03_the_ethernet_frame`,
`s04_error_detection_and_the_fcs`

**Ch 16 — Ethernet.** ALOHAnet between Hawaiian islands; Metcalfe's 1973 memo and
CSMA/CD; the standards ladder from 10BASE5 to 400GBASE and what changed at each
rung; duplex, autonegotiation and Power over Ethernet.
*Sections:* `s01_aloha_and_the_hawaiian_packet`, `s02_metcalfes_memo_and_csma_cd`,
`s03_the_standards_ladder`, `s04_duplex_autonegotiation_and_poe`

**Ch 17 — Switches.** Hubs, bridges and switches as one evolutionary line; the
MAC address table, learning and flooding; collision and broadcast domains;
store-and-forward vs. cut-through, buffers and microbursts.
*Sections:* `s01_hubs_bridges_and_switches`, `s02_the_mac_address_table`,
`s03_collision_and_broadcast_domains`, `s04_forwarding_modes_and_buffers`

**Ch 18 — ARP.** Two address worlds and the gap between them; the request/reply
exchange packet by packet; the ARP cache, gratuitous ARP, proxy ARP and their
failure modes; IPv6 Neighbor Discovery as the redesign.
*Sections:* `s01_two_address_worlds`, `s02_the_arp_exchange`,
`s03_the_arp_cache_and_its_failures`, `s04_ipv6_neighbor_discovery`

**Ch 19 — Loops and Spanning Tree.** The broadcast storm as a first-hand disaster;
Radia Perlman's algorithm and the poem she wrote about it; RSTP, MSTP and modern
loop protection (BPDU guard, root guard); link aggregation and LACP.
*Sections:* `s01_the_broadcast_storm`, `s02_perlmans_algorhyme`,
`s03_rstp_mstp_and_modern_practice`, `s04_link_aggregation`

**Ch 20 — VLANs.** The case for logical segmentation; 802.1Q tagging bit by bit;
access ports, trunks, the native VLAN and its security consequence; inter-VLAN
routing, router-on-a-stick, SVIs and voice VLANs.
*Sections:* `s01_the_case_for_logical_segmentation`, `s02_dot1q_tagging`,
`s03_access_ports_trunks_and_native_vlans`, `s04_inter_vlan_routing_and_voice_vlans`

---

## Unit V — The Idea of Layers

*Chapters 21–23. Directory: `book/unit_05_layering/`*

Only now, with four solved problems behind us, does abstraction become obviously
necessary rather than merely traditional.

**Ch 21 — Why Layering Exists.** The combinatorial argument (m applications × n
media); interfaces and service primitives; the real cost of abstraction; layer
violations that ship anyway.
*Sections:* `s01_the_combinatorial_argument`, `s02_interfaces_and_service_primitives`,
`s03_the_cost_of_abstraction`, `s04_layer_violations_in_the_wild`

**Ch 22 — The OSI Model.** The ISO committee, the chart, and why the model
outlived the protocols; layers 1–3 mapped onto everything already built; layers
4–7; OSI as a *diagnostic instrument* — the use that actually justifies memorising it.
*Sections:* `s01_the_committee_and_the_chart`, `s02_layers_one_to_three`,
`s03_layers_four_to_seven`, `s04_using_osi_as_a_diagnostic_tool`

**Ch 23 — TCP/IP and Encapsulation.** Cerf and Kahn's 1974 paper; the four-layer
model and its deliberate refusal to specify; encapsulation and PDUs traced through
one real HTTP request; the end-to-end argument and the hourglass.
*Sections:* `s01_cerf_kahn_and_the_internet_protocol_suite`, `s02_the_four_layer_model`,
`s03_encapsulation_and_pdus`, `s04_the_end_to_end_argument`

---

## Unit VI — Addressing the World

*Chapters 24–28. Directory: `book/unit_06_addressing_the_internet/`*

The mathematical heart of the course. Subnetting is derived, never memorised.

**Ch 24 — The Internet Protocol.** Best-effort delivery as a design *choice*;
the IPv4 header field by field; fragmentation, MTU and why it is a mistake we
still pay for; TTL as loop insurance.
*Sections:* `s01_best_effort_delivery`, `s02_the_ipv4_header`,
`s03_fragmentation_and_mtu`, `s04_ttl_and_loop_survival`

**Ch 25 — IPv4 Addresses and Masks.** Dotted decimal as a human convenience over a
32-bit integer; the network/host split; the subnet mask as a bitwise operator;
classful addressing as history that still haunts defaults.
*Sections:* `s01_dotted_decimal`, `s02_network_and_host_portions`,
`s03_the_subnet_mask`, `s04_classful_history_and_its_ghosts`

**Ch 26 — Subnetting, CIDR, and VLSM.** Borrowing bits; a complete worked
`192.168.10.70/27` in binary, then the mental shortcuts, then a page of drilled
examples; CIDR and route aggregation; VLSM and hierarchical plans that summarise.
*Sections:* `s01_borrowing_bits`, `s02_working_a_subnet_by_hand`,
`s03_cidr_and_supernetting`, `s04_vlsm_and_hierarchical_plans`

**Ch 27 — Address Plans in Practice.** RFC 1918 and the private ranges; loopback,
APIPA/link-local, documentation ranges, CGNAT space; unicast, broadcast, multicast
and anycast; IPAM and what a defensible plan document contains.
*Sections:* `s01_private_space_and_rfc1918`, `s02_special_ranges_and_apipa`,
`s03_unicast_broadcast_multicast_anycast`, `s04_ipam_and_documenting_a_plan`

**Ch 28 — IPv6.** The exhaustion arithmetic and the exact dates it came due;
notation, compression, and address types (GUA, ULA, link-local, multicast);
SLAAC, DHCPv6 and NDP; dual-stack, tunnelling and NAT64 in the long transition.
*Sections:* `s01_the_exhaustion_arithmetic`, `s02_notation_and_address_types`,
`s03_slaac_dhcpv6_and_neighbor_discovery`, `s04_transition_and_coexistence`

---

## Unit VII — Finding the Way

*Chapters 29–34. Directory: `book/unit_07_routing/`*

**Ch 29 — Forwarding and Longest-Prefix Match.** The single decision a router
makes, stated precisely; reading a real routing table on Linux and on IOS;
longest-prefix match derived before it is named; the default gateway as the
route of last resort.
*Sections:* `s01_the_forwarding_decision`, `s02_reading_a_routing_table`,
`s03_longest_prefix_match`, `s04_the_default_gateway`

**Ch 30 — Static Routing.** Configuring a route on three platforms; metrics and
administrative distance; default and floating static routes; the operational point
at which static routing stops scaling.
*Sections:* `s01_configuring_a_static_route`, `s02_metrics_and_administrative_distance`,
`s03_default_and_floating_routes`, `s04_when_static_stops_scaling`

**Ch 31 — Dynamic Routing.** Distance vector, Bellman–Ford and RIP; count-to-
infinity and the fixes (split horizon, poison reverse, holddown); link state,
Dijkstra and OSPF; convergence, areas, and how design follows the algorithm.
*Sections:* `s01_distance_vector_and_rip`, `s02_the_count_to_infinity_problem`,
`s03_link_state_ospf_and_dijkstra`, `s04_convergence_areas_and_design`

**Ch 32 — BGP and Autonomous Systems.** The AS as an administrative, not
technical, boundary; path vector and policy routing; peering, transit and the
money that shapes routes; route leaks and hijacks, RPKI and the fragility of the
global table.
*Sections:* `s01_autonomous_systems`, `s02_path_vector_and_policy`,
`s03_peering_transit_and_money`, `s04_route_leaks_and_hijacks`

**Ch 33 — NAT and PAT.** The 1994 workaround that bought thirty years; static,
dynamic and overload (PAT) with a worked translation table; what NAT breaks and
the protocols invented to work around it; CGNAT and the IPv6 endgame.
*Sections:* `s01_the_address_shortage_workaround`, `s02_static_dynamic_and_overload`,
`s03_what_nat_breaks`, `s04_carrier_grade_nat_and_ipv6`

**Ch 34 — ICMP and the Diagnostic Path.** IP's error channel; echo request/reply
and what `ping` actually proves; the TTL trick that makes `traceroute` work, in
both UDP and ICMP flavours; path MTU discovery and the black-hole failure that
still bites in 2026.
*Sections:* `s01_the_error_channel`, `s02_ping_and_echo`,
`s03_traceroute_and_the_ttl_trick`, `s04_path_mtu_discovery_and_black_holes`

---

## Unit VIII — Reaching the Application

*Chapters 35–41. Directory: `book/unit_08_transport_and_services/`*

**Ch 35 — Ports and Process Multiplexing.** The final-hop problem stated before
any port number is shown; the socket as a five-tuple; well-known/registered/
ephemeral ranges; reading socket state with `ss` and `netstat`.
*Sections:* `s01_the_final_hop_problem`, `s02_the_socket`,
`s03_well_known_registered_ephemeral`, `s04_reading_socket_state`

**Ch 36 — UDP.** Eight bytes of header, examined; when speed and simplicity beat
certainty; the application classes that chose UDP and why; datagram hazards
(amplification, fragmentation, no congestion response).
*Sections:* `s01_the_minimal_header`, `s02_when_speed_beats_certainty`,
`s03_udp_applications`, `s04_datagram_hazards`

**Ch 37 — TCP.** The three-way handshake as a mutual-agreement protocol;
sequence numbers and cumulative acknowledgement; retransmission, RTO estimation
and fast retransmit; the sliding window and flow control; teardown and the state
machine including TIME_WAIT.
*Sections:* `s01_the_three_way_handshake`, `s02_sequence_numbers_and_acknowledgement`,
`s03_retransmission_and_timers`, `s04_flow_control_and_the_window`,
`s05_teardown_and_state_machine`

**Ch 38 — Congestion Control and Modern Transports.** The October 1986 collapse
of the NSFNET backbone from 32 kb/s to 40 b/s; slow start, AIMD and the sawtooth;
CUBIC, BBR and ECN; QUIC, HTTP/3 and the migration of transport into userspace.
*Sections:* `s01_the_1986_congestion_collapse`, `s02_slow_start_and_aimd`,
`s03_cubic_bbr_and_ecn`, `s04_quic_and_http3`

**Ch 39 — DNS.** HOSTS.TXT, one file, one maintainer, and the moment it broke;
the resolution walk from stub resolver to root to TLD to authoritative; record
types and zone files; caching, TTL, negative caching, DNSSEC, DoH/DoT.
*Sections:* `s01_from_hosts_txt_to_hierarchy`, `s02_the_resolution_walk`,
`s03_record_types_and_zones`, `s04_caching_ttl_and_dns_security`

**Ch 40 — DHCP.** Why manual configuration fails at n = 200; DORA packet by
packet; scopes, reservations, exclusions and the option catalogue that quietly
runs the enterprise; relay agents, and the failure modes (rogue servers, pool
exhaustion, lease conflicts).
*Sections:* `s01_manual_configuration_does_not_scale`, `s02_dora`,
`s03_scopes_reservations_and_options`, `s04_relays_and_failure_modes`

**Ch 41 — The Application Protocol Zoo.** HTTP/HTTPS and the TLS handshake in
place; SSH, FTP/SFTP/TFTP and remote access; SMTP/IMAP/POP3, LDAP, NTP; SIP/RTP
for voice, SNMP and syslog for management. Ports attached to protocols only after
each protocol's purpose is clear.
*Sections:* `s01_the_web_http_and_tls`, `s02_remote_access_and_file_transfer`,
`s03_mail_directory_and_time`, `s04_voice_video_and_management`

---

## Unit IX — Networking Without Wires

*Chapters 42–47. Directory: `book/unit_09_wireless/`*

Built from the electromagnetic field upward, so that "channel 6 is congested" is
a physical statement rather than a piece of folklore.

**Ch 42 — Radio from First Principles.** The electromagnetic wave and what
oscillates; frequency, wavelength and the antenna sized to a fraction of it;
free-space path loss and a complete link budget in dBm; reflection, multipath,
fading and the Fresnel zone.
*Sections:* `s01_the_electromagnetic_wave`, `s02_frequency_wavelength_and_antennas`,
`s03_path_loss_and_the_link_budget`, `s04_reflection_multipath_and_fading`

**Ch 43 — Spectrum and Channels.** Regulation, licensed vs. unlicensed, and the
ISM accident that gave us Wi-Fi; channels, width and the arithmetic of overlap;
2.4, 5 and 6 GHz compared on range, capacity and crowding; the noise floor and
what raises it.
*Sections:* `s01_regulation_and_the_ism_bands`, `s02_channels_width_and_overlap`,
`s03_the_bands_compared`, `s04_interference_and_the_noise_floor`

**Ch 44 — Wi-Fi.** The 802.11 family a→be with the data rate and the reason for
each; CSMA/CA, the hidden node problem and RTS/CTS; management, control and data
frames, SSID/BSSID, association and the four-way handshake; MIMO, MU-MIMO, OFDMA
and beamforming.
*Sections:* `s01_the_802_11_family`, `s02_csma_ca_and_the_hidden_node`,
`s03_frames_ssids_and_association`, `s04_mimo_ofdma_and_modern_phy`

**Ch 45 — WLAN Design and Troubleshooting.** Site surveys (predictive, passive,
active) and AP placement; roaming, 802.11k/v/r, controllers and cloud management;
capacity vs. coverage design; a diagnostic procedure for the four complaints users
actually make.
*Sections:* `s01_site_surveys_and_ap_placement`, `s02_roaming_and_controllers`,
`s03_capacity_versus_coverage`, `s04_diagnosing_wireless_complaints`

**Ch 46 — Cellular.** The 1947 Bell Labs cellular idea and frequency reuse;
1G analog to 3G packet data; LTE and the all-IP core; 5G NR, mmWave, network
slicing and private cellular in the enterprise.
*Sections:* `s01_the_cellular_idea`, `s02_1g_to_3g_the_digital_turn`,
`s03_lte_and_the_all_ip_core`, `s04_5g_nr_slicing_and_private_networks`

**Ch 47 — Short-Range and IoT Radio.** Bluetooth and BLE; Zigbee, Thread and
Matter; LPWAN — LoRaWAN and NB-IoT and the range/rate/power trilemma; NFC, RFID
and the very short range.
*Sections:* `s01_bluetooth_and_ble`, `s02_zigbee_thread_and_matter`,
`s03_lpwan_lora_and_nb_iot`, `s04_nfc_rfid_and_the_very_short_range`

---

## Unit X — Networks at Distance

*Chapters 48–52. Directory: `book/unit_10_networks_at_distance/`*

**Ch 48 — Internet Architecture.** The real shape of the Internet (not a cloud);
peering, transit, IXPs and the settlement-free handshake; IANA, the RIRs and where
addresses actually come from; the IETF, RFCs, and governance by rough consensus.
*Sections:* `s01_the_shape_of_the_internet`, `s02_peering_transit_and_ixps`,
`s03_iana_and_the_rirs`, `s04_governance_and_standards`

**Ch 49 — The Last Mile.** Dial-up and the 56k ceiling that Shannon predicted;
DSL and the copper reuse trick; DOCSIS and cable's shared segment; PON/FTTH and
the splitter; fixed wireless, GEO latency arithmetic and the LEO constellations.
*Sections:* `s01_dial_up_and_dsl`, `s02_cable_and_docsis`,
`s03_fiber_to_the_home_and_pon`, `s04_fixed_wireless_and_satellite`

**Ch 50 — Carrier and Optical Transport.** Leased lines and the T/E hierarchy;
SONET/SDH rings, and OTN; DWDM, the C-band and the erbium amplifier that made
long haul economic; MPLS and label switching; submarine cables and the physical
geography of the Internet.
*Sections:* `s01_leased_lines_and_the_digital_hierarchy`, `s02_sonet_sdh_and_otn`,
`s03_dwdm_and_the_optical_core`, `s04_mpls_and_label_switching`,
`s05_submarine_cables`

**Ch 51 — Enterprise WAN Evolution.** From Frame Relay and ATM to broadband
underlay; SD-WAN and policy-driven path selection; direct cloud interconnect;
designing for the branch office and the permanently remote workforce.
*Sections:* `s01_from_frame_relay_to_broadband`, `s02_sd_wan`,
`s03_direct_cloud_interconnect`, `s04_designing_for_branch_and_remote_work`

**Ch 52 — QoS and Content Delivery.** Why best-effort fairness fails a voice call;
classification, DSCP marking and queue disciplines; policing, shaping, and the
buffer-sizing question; caching, CDNs and anycast as the other half of the answer.
*Sections:* `s01_why_fairness_is_not_enough`, `s02_classification_marking_and_queues`,
`s03_policing_shaping_and_buffers`, `s04_caching_cdns_and_anycast`

---

## Unit XI — Operating a Network

*Chapters 53–56. Directory: `book/unit_11_operations/`*

A network is not finished when packets flow. This unit is deliberately expanded
beyond the traditional syllabus, matching the 19% operations weight in N10-009.

**Ch 53 — Documentation and IPAM.** The three diagrams (physical, logical,
L3/routed) and what belongs on each; inventory, rack elevations, cable maps and
labelling standards; address management; runbooks and the knowledge base.
*Sections:* `s01_the_three_diagrams`, `s02_inventory_racks_and_labels`,
`s03_address_management`, `s04_runbooks_and_the_knowledge_base`

**Ch 54 — Monitoring and Telemetry.** Baselines and the meaning of "normal";
SNMP versions, MIBs, OIDs, polls and traps; syslog severities and correlation;
NetFlow/IPFIX/sFlow and streaming telemetry; alert design that people don't ignore.
*Sections:* `s01_baselines_and_what_normal_means`, `s02_snmp`,
`s03_logging_syslog_and_correlation`, `s04_flow_records_and_streaming_telemetry`

**Ch 55 — Configuration and Change Management.** Configuration as accumulated
liability; change control, windows and rollback plans; lifecycle, firmware,
patching, EOL/EOS; backups, golden configs and configuration drift detection.
*Sections:* `s01_configuration_as_a_liability`, `s02_change_control`,
`s03_lifecycle_patching_and_eol`, `s04_backups_and_rollback`

**Ch 56 — Availability and Recovery.** Nines and what each costs; redundancy,
FHRP (VRRP/HSRP/GLBP), and the failure of redundancy that shares a fate; power,
cooling and the physical plant; DR sites, RPO/RTO and testing the plan.
*Sections:* `s01_measuring_availability`, `s02_redundancy_and_first_hop_protection`,
`s03_power_cooling_and_the_physical_plant`, `s04_disaster_recovery_rpo_and_rto`

---

## Unit XII — Securing a Network

*Chapters 57–62. Directory: `book/unit_12_security/`*

Derived from a single question: *what can an adversary do to the system we just
built?* Listening, altering, and preventing — from which the CIA triad falls out
rather than being asserted.

**Ch 57 — Threat Models and the CIA Triad.** Who attacks a network and what they
want; confidentiality, integrity and availability derived from the three verbs;
assets, risk and proportionate defence; the attack surface enumerated layer by layer.
*Sections:* `s01_who_attacks_a_network_and_why`, `s02_confidentiality_integrity_availability`,
`s03_assets_risk_and_proportionality`, `s04_the_attack_surface_of_the_stack`

**Ch 58 — Cryptography for Network Engineers.** Symmetric ciphers and the key
distribution problem; Diffie–Hellman and public key; hashes, MACs and signatures;
certificates, chains of trust, PKI and the TLS 1.3 handshake in full.
*Sections:* `s01_symmetric_ciphers`, `s02_public_key_and_key_exchange`,
`s03_hashes_macs_and_signatures`, `s04_certificates_pki_and_tls`

**Ch 59 — Authentication, Authorization, Accounting.** Identity and credential
factors; 802.1X, EAP methods, RADIUS and TACACS+; authorization models and least
privilege; NAC, posture, and the zero-trust reframing.
*Sections:* `s01_identity_and_credentials`, `s02_802_1x_and_radius`,
`s03_authorization_models_and_least_privilege`, `s04_zero_trust`

**Ch 60 — Firewalls, ACLs, and Segmentation.** The ACL as a match-action list and
the implicit deny; stateful inspection and the connection table; NGFW, IDS/IPS,
proxies and TLS inspection's tradeoff; DMZs, segmentation and microsegmentation.
*Sections:* `s01_the_access_control_list`, `s02_stateful_inspection`,
`s03_next_generation_firewalls_and_proxies`, `s04_segmentation_dmzs_and_microsegmentation`

**Ch 61 — VPNs and Secure Remote Access.** The tunnel idea and what encapsulation
buys; IPsec, AH/ESP, IKE, transport vs. tunnel mode; TLS VPNs and WireGuard's
minimalism; split tunnelling, remote-access design and the post-2020 reality.
*Sections:* `s01_the_tunnel_idea`, `s02_ipsec`, `s03_tls_vpns_and_wireguard`,
`s04_remote_access_design`

**Ch 62 — Attacks on the Stack.** MAC flooding, VLAN hopping, rogue DHCP and STP
attacks with their switch-level mitigations; ARP spoofing, DNS poisoning, on-path
and downgrade attacks; DoS/DDoS, reflection and amplification arithmetic; defence
in depth and a device hardening checklist.
*Sections:* `s01_layer_two_attacks`, `s02_spoofing_poisoning_and_on_path`,
`s03_denial_of_service`, `s04_defense_in_depth_and_hardening`

---

## Unit XIII — Diagnosis

*Chapters 63–66. Directory: `book/unit_13_troubleshooting/`*

Formalising a method the reader has been using since Unit II. 24% of N10-009 and,
more importantly, most of the job.

**Ch 63 — The Methodology.** Evidence before hypothesis, and the cost of the
reverse; the seven-step process (identify, theorise, test, plan, implement,
verify, document); divide-and-conquer by layer, top-down, bottom-up, follow-the-
path; documenting so the next failure is cheaper.
*Sections:* `s01_evidence_before_hypothesis`, `s02_the_seven_step_process`,
`s03_divide_and_conquer_by_layer`, `s04_documenting_and_learning`

**Ch 64 — The Toolbox.** `ping`, `traceroute`/`tracert`, `mtr`; `ip`/`ifconfig`,
`arp`, `nslookup`/`dig`; `tcpdump` and Wireshark, filters and reading a capture;
`iperf`, `nmap`, cable testers, tone generators, OTDRs, Wi-Fi analysers, and the
`show` commands that matter.
*Sections:* `s01_reachability_tools`, `s02_name_and_address_tools`,
`s03_packet_capture`, `s04_performance_and_discovery_tools`

**Ch 65 — Failure Modes, Layer by Layer.** A structured catalogue tied to symptom:
physical (no link, bad pinout, attenuation, dirty fibre); data link (wrong VLAN,
native mismatch, STP block, duplex); network (wrong mask, wrong gateway, missing
route, NAT); transport and services (blocked port, DHCP exhaustion, DNS).
*Sections:* `s01_physical_failures`, `s02_data_link_failures`,
`s03_network_and_routing_failures`, `s04_transport_and_service_failures`

**Ch 66 — Performance Problems.** Separating bandwidth, latency and loss from a
single user complaint; duplex mismatch, CRC errors and the interface counters that
prove it; MTU, fragmentation and PMTUD black holes; bufferbloat, AQM (CoDel, FQ)
and why more buffer made things worse.
*Sections:* `s01_separating_bandwidth_latency_and_loss`, `s02_duplex_mismatch_and_errors`,
`s03_mtu_fragmentation_and_black_holes`, `s04_bufferbloat_and_queue_management`

---

## Unit XIV — Modern and Future Networks

*Chapters 67–72. Directory: `book/unit_14_modern_and_future/`*

Where the book looks past any one semester.

**Ch 67 — Virtualization and Overlays.** Virtual switches, VM and container
networking (CNI, namespaces); VXLAN and GENEVE encapsulation and the 24-bit VNI;
EVPN as the control plane that made overlays operable; leaf–spine fabrics and the
east–west traffic shift that forced them.
*Sections:* `s01_virtual_switches_and_container_networking`, `s02_vxlan_and_geneve`,
`s03_evpn_and_the_control_plane`, `s04_leaf_spine_data_center_fabrics`

**Ch 68 — SDN and Programmable Networks.** Separating control and data planes;
OpenFlow, the controller, and what the first decade got wrong; P4 and programmable
pipelines; intent-based networking and the honest limits of the claim.
*Sections:* `s01_separating_control_and_data_planes`, `s02_openflow_and_the_controller`,
`s03_p4_and_programmable_pipelines`, `s04_intent_based_networking`

**Ch 69 — Cloud Networking.** Service and deployment models stated precisely; the
VPC, its subnets, route tables, gateways and security groups; hybrid and
multicloud connectivity (VPN, direct connect, transit gateways); cloud-native
load balancing, ingress and the service mesh.
*Sections:* `s01_service_and_deployment_models`, `s02_the_virtual_private_cloud`,
`s03_hybrid_and_multicloud_connectivity`, `s04_cloud_native_load_balancing_and_service_mesh`

**Ch 70 — Automation and Infrastructure as Code.** Why the CLI does not scale past
a few dozen devices; APIs, NETCONF/RESTCONF, YANG and gNMI; Ansible, Terraform and
declarative state; CI/CD for network configuration, testing, and the honest state
of AIOps.
*Sections:* `s01_why_the_cli_does_not_scale`, `s02_apis_netconf_restconf_and_yang`,
`s03_ansible_terraform_and_declarative_state`, `s04_ci_cd_testing_and_aiops`

**Ch 71 — The Frontier.** 6G research directions and what is and is not plausible;
coherent optics, 800G/1.6T and the shrinking margin; quantum networking, QKD and
the entanglement-distribution problem; TSN and deterministic networking; machine
learning in the network and on the network.
*Sections:* `s01_beyond_5g`, `s02_terabit_optics_and_coherent_transmission`,
`s03_quantum_networking_and_qkd`, `s04_deterministic_networking_and_tsn`,
`s05_ai_in_and_on_the_network`

**Ch 72 — Network Design: The Synthesis.** Requirements elicitation — who talks to
whom, about what, how badly does it matter; from requirements to topology, media
and capacity; addressing, services and security designed together rather than
bolted on; defending the design, which is the skill the whole book has been for.
*Sections:* `s01_requirements_elicitation`, `s02_from_requirements_to_topology`,
`s03_addressing_services_and_security_together`, `s04_defending_the_design`

---

## Appendices

`book/appendices/`

- **A — Binary, Hex, and Subnetting Reference.** Conversion tables, powers of two
  to 2³², the /8–/32 mask table, and a one-page subnetting procedure.
- **B — Ports, Protocols, and Standards.** The port table, arranged by *purpose*
  rather than number; the IEEE 802 and IETF documents cited in the book.
- **C — Standards Bodies and How a Standard Happens.** IEEE, IETF, ITU-T, ISO,
  ANSI/TIA, 3GPP, Wi-Fi Alliance — who does what and why it matters.
- **D — CompTIA Network+ N10-009 Crosswalk.** Every exam objective mapped to the
  chapter and section that derives it.
- **E — Glossary.** Every marked term in the book, with the chapter that
  introduces it.
- **F — A Timeline of Telecommunications.** 1837 to the present, the dates cited
  in the narrative, in order.

---

## Per-Chapter Apparatus

Every chapter directory carries, without exception:

| File | Contents |
|---|---|
| `chapter_intro.md` | The opening scene, the problem, the chapter's place in the spine, and what the reader will be able to do afterwards |
| `sNN_<slug>.md` | The lesson prose, 1,200–2,000 words each |
| `exercises.md` | 10–16 problems graded *Warm-up / Working / Challenge / Design*, plus a "diagnose this" scenario |
| `important_concepts.md` | The chapter's terms, each with a one-paragraph definition and the section that derives it |
| `important_researchers.md` | The people behind the chapter, with real dates and real contributions |
| `further_reading.md` | Annotated bibliography: primary sources (RFCs, IEEE standards, original papers) and accessible secondary reading |

---

## Companion Assets

| Directory | Contents |
|---|---|
| `labs/` | 15 hands-on lab guides, one per course week, each with objectives, prerequisites, procedure, expected observations, and debrief questions |
| `tools/` | Runnable Python: subnet/VLSM calculator and practice generator, CIDR drill, encoding and modulation visualisers, CSMA simulator, queueing and link-budget calculators, routing-table longest-prefix demo, Wi-Fi channel planner |
| `project/` | The semester-long Network Design and Technical Justification project: brief, seven staged deliverables, and rubrics |
| `instructor/` | 15-week schedule mapped to chapters and labs, three exam blueprints, retrieval-quiz banks, and the Network+ objective crosswalk |

---

## Quiz Mapping

`subject.toml` maps all 72 chapters, grouped into **six phases**:

| Phase | Name | Chapters |
|---|---|---|
| 0 | Foundations of Information and Signals | 1–10 |
| 1 | Sharing Infrastructure and Local Delivery | 11–23 |
| 2 | Addressing and Routing | 24–34 |
| 3 | Transport, Services, and Wireless | 35–47 |
| 4 | Wide Area, Operations, and Security | 48–62 |
| 5 | Diagnosis, Modern Practice, and Design | 63–72 |
