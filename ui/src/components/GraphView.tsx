import { useCallback, useEffect, useState } from "react";
import ReactFlow, {
  Node,
  Edge,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
} from "reactflow";
import "reactflow/dist/style.css";
import { ProgramAnalysis } from "../types";

interface GraphViewProps {
  analysis: ProgramAnalysis;
  currentStep: number;
}

export default function GraphView({ analysis, currentStep }: GraphViewProps) {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  useEffect(() => {
    const newNodes: Node[] = [];
    const newEdges: Edge[] = [];
    const nodePositions = new Map<string, { x: number; y: number }>();

    let yOffset = 0;
    const xSpacing = 250;
    const ySpacing = 100;

    analysis.variables.forEach((variable, index) => {
      const nodeId = `var-${variable.id[0]}`;
      const x = (index % 3) * xSpacing;
      const y = Math.floor(index / 3) * ySpacing;

      nodePositions.set(nodeId, { x, y });

      const eventsForVar = analysis.events.filter(
        (e) => e.variable_id[0] === variable.id[0] && analysis.events.indexOf(e) <= currentStep
      );

      const lastEvent = eventsForVar[eventsForVar.length - 1];
      const nodeColor = lastEvent ? getNodeColor(lastEvent.kind) : "#6b7280";

      newNodes.push({
        id: nodeId,
        type: "default",
        position: { x, y },
        data: {
          label: (
            <div className="text-center">
              <div className="font-mono font-bold">{variable.name}</div>
              <div className="text-xs text-gray-400">{variable.ty}</div>
              {lastEvent && (
                <div className="text-xs mt-1 text-gray-300">{lastEvent.kind}</div>
              )}
            </div>
          ),
        },
        style: {
          background: nodeColor,
          color: "white",
          border: "2px solid #1f2937",
          borderRadius: "8px",
          padding: "10px",
          minWidth: "120px",
        },
      });
    });

    analysis.ownership_graph.edges.forEach((edge, index) => {
      const sourceId = getNodeId(edge.source);
      const targetId = getNodeId(edge.target);

      if (sourceId && targetId) {
        const edgeColor = getEdgeColor(edge.kind);
        const edgeLabel = edge.kind.replace(/([A-Z])/g, " $1").trim();

        newEdges.push({
          id: `edge-${index}`,
          source: sourceId,
          target: targetId,
          label: edgeLabel,
          type: "smoothstep",
          animated: true,
          style: { stroke: edgeColor, strokeWidth: 2 },
          labelStyle: { fill: edgeColor, fontWeight: 600, fontSize: 12 },
        });
      }
    });

    setNodes(newNodes);
    setEdges(newEdges);
  }, [analysis, currentStep, setNodes, setEdges]);

  const getNodeId = (node: any): string | null => {
    if (node.Variable) return `var-${node.Variable[0]}`;
    if (node.Reference) return `ref-${node.Reference[0]}`;
    if (node.Function) return `fn-${node.Function[0]}`;
    if (node.Scope) return `scope-${node.Scope[0]}`;
    return null;
  };

  return (
    <div className="h-full bg-gray-900">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        fitView
      >
        <Background color="#374151" gap={16} />
        <Controls />
        <MiniMap
          nodeColor={(node) => node.style?.background as string}
          maskColor="rgba(0, 0, 0, 0.6)"
        />
      </ReactFlow>
    </div>
  );
}

function getNodeColor(kind: string): string {
  switch (kind) {
    case "Create":
    case "StorageLive":
      return "#22c55e";
    case "MoveOut":
      return "#ef4444";
    case "BorrowShared":
      return "#06b6d4";
    case "BorrowMut":
      return "#eab308";
    case "Drop":
    case "StorageDead":
      return "#a855f7";
    default:
      return "#6b7280";
  }
}

function getEdgeColor(kind: string): string {
  switch (kind) {
    case "Owns":
      return "#22c55e";
    case "Borrows":
      return "#06b6d4";
    case "MutablyBorrows":
      return "#eab308";
    case "MovesTo":
      return "#ef4444";
    case "Reborrows":
      return "#8b5cf6";
    case "DropsAt":
      return "#a855f7";
    default:
      return "#6b7280";
  }
}
