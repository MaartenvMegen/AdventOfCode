import networkx as nx
import matplotlib.pyplot as plt
G = nx.Graph()

G.add_node( "G",  color= "red" )
G.add_node( "B",  color= "red" )
G.add_node( "C",  color= "green" )
G.add_node( "C",  color= "green" )
G.add_node( "D",  color= "green" )
G.add_node( "E",  color= "green" )
G.add_node( "F",  color= "green" )

G.add_edge("C", "B", weight=7)
G.add_edge("G","B", weight=1)
G.add_edge("G","D", weight=2)
G.add_edge("D","E", weight=1)
G.add_edge("D","F", weight=1)
G.add_edge("E","B", weight=5)


print(f"Number of nodes {G.number_of_nodes()}")
edge_labels = {(u, v): d['weight'] for u, v, d in G.edges(data=True)}

fig = plt.figure()
pos = nx.spring_layout(G)  # positions for all nodes
nx.draw(G, pos=pos, with_labels=True, font_weight='bold')
nx.draw_networkx_edge_labels(G, pos, edge_labels, font_size=20, font_family="sans-serif")

print(f"shortest path from E to C is {nx.shortest_path(G, 'E', 'C')} with length {nx.shortest_path_length(G, 'E', 'C')}")
plt.show()